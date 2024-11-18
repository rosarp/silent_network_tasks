use axum::{routing::any, Router};
use dashmap::DashMap;
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::mpsc::Sender;
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::EnvFilter;

mod handler;

type Who = String;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(Level::INFO)
        .init();

    let user_map: Arc<DashMap<Who, UserState>> = Arc::new(DashMap::new());

    let assets_dir = PathBuf::from("./assets");

    let app = Router::new()
        .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
        .route(
            "/wait-for-second-party/:unique_id",
            any(handler::ws_handler),
        )
        .layer(
            TraceLayer::new_for_http().make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::INFO)
                    .include_headers(true),
            ),
        )
        .with_state(user_map);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

pub struct UserState {
    channel_id: u32,
    sender: Sender<String>,
}
