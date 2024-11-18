use std::{borrow::Cow, net::SocketAddr, ops::ControlFlow, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use axum_extra::{headers, TypedHeader};
use dashmap::DashMap;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::{self};
use tracing::info;

use crate::{UserState, Who};

// Note: Picked websocket routine from axum examples

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    user_state: State<Arc<DashMap<Who, UserState>>>,
    channel_id: Path<u32>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    info!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr, user_state, channel_id))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(
    socket: WebSocket,
    who: SocketAddr,
    State(user_state): State<Arc<DashMap<Who, UserState>>>,
    Path(channel_id): Path<u32>,
) {
    // Create new mpsc channel for this user
    let (mpsc_sender, mut mpsc_receiver) = mpsc::channel(100);

    user_state.insert(
        who.to_string(),
        UserState {
            channel_id,
            sender: mpsc_sender.clone(),
        },
    );

    // By splitting socket we can send and receive at the same time. In this example we will send
    // unsolicited messages to client based on some sort of server's internal event (i.e .timer).
    let (mut sender, mut receiver) = socket.split();

    let ust_receiver_clone = user_state.clone();
    // Spawn a task that will push messages from other users to the client {who}
    let mut send_task = tokio::spawn(async move {
        let mut total_millis = 10000;
        while ust_receiver_clone.len() < 2 {
            // Disconnect if we don't get a message for 10 seconds
            if total_millis < 1 {
                ust_receiver_clone.remove(&who.to_string());
                info!("User {who} disconnected");
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            total_millis -= 100;
        }

        // There are now more than 1 user connected
        if sender
            .send(Message::Text(format!("Connected...")))
            .await
            .is_err()
        {
            return;
        }
        // Receive message from other users
        while let Some(msg) = mpsc_receiver.recv().await {
            // In case of any websocket error, we exit.
            if sender.send(Message::Text(format!("{msg}"))).await.is_err() {
                return;
            }
        }
    });

    // This second task will receive messages from client {who} and send to other users
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            let ust_sender_clone = user_state.clone();

            // send message to other users and break if instructed to do so
            if process_message(msg, who, ust_sender_clone, channel_id)
                .await
                .is_break()
            {
                break;
            }
        }
    });

    // If any one of the tasks exit, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(_) => info!("messages sent to {who}"),
                Err(a) => info!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(_) => info!("Received messages"),
                Err(b) => info!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    info!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
async fn process_message(
    msg: Message,
    who: SocketAddr,
    ust_sender_clone: Arc<DashMap<Who, UserState>>,
    channel_id: u32,
) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            info!(">>> {who} sent str: {t:?}");
            for state in ust_sender_clone.iter() {
                if state.value().channel_id != channel_id {
                    continue;
                }
                // TODO: report back that this message was not delivered
                _ = state
                    .value()
                    .sender
                    .send(format!("{who}: {t}"))
                    .await
                    .is_err();
            }
        }
        Message::Binary(d) => {
            info!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                info!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                info!(">>> {who} somehow sent close message without CloseFrame");
            }
            ust_sender_clone.remove(&who.to_string());
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            info!(">>> {who} sent pong with {v:?}");
        }
        // You should never need to manually handle Message::Ping, as axum's websocket library
        // will do so for you automagically by replying with Pong and copying the v according to
        // spec. But if you need the contents of the pings you can see them here.
        Message::Ping(v) => {
            info!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
