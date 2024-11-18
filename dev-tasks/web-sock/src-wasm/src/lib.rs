use futures::channel::oneshot;
use wasm_bindgen::prelude::*;
use web_sys::{console::log_1, BinaryType::Arraybuffer, ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log_1(&JsValue::from_str(&format_args!($($t)*).to_string())))
}

#[wasm_bindgen]
pub async fn wsPing(endpoint: String, message: String) -> Result<JsValue, JsValue> {
    console_log!("Connecting to WebSocket endpoint: {}", endpoint);
    let web_socket = WebSocket::new(&endpoint)?;
    web_socket.set_binary_type(Arraybuffer);

    let (sender, receiver) = oneshot::channel();
    let sender = std::rc::Rc::new(std::cell::RefCell::new(Some(sender)));

    let sender_clone = sender.clone();
    // Set up message handler
    let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
        if let Some(sender) = sender_clone.borrow_mut().take() {
            let response = e.data();
            console_log!("Received response: {:?}", response);
            let _ = sender.send(Ok(response));
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    web_socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
    onmessage_callback.forget();

    // Set up error handler
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        if let Some(sender) = sender.borrow_mut().take() {
            console_log!("WebSocket error occurred: {:?}", e);
            let _ = sender.send(Err(e.error()));
        }
    }) as Box<dyn FnMut(ErrorEvent)>);

    web_socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();

    let cloned_ws = web_socket.clone();
    // Set up open handler
    let onopen_callback = Closure::wrap(Box::new(move || {
        console_log!("WebSocket connection opened");
        // Send the message
        _ = cloned_ws.send_with_str(&message).map_err(|e| e);
    }) as Box<dyn FnMut()>);

    web_socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
    onopen_callback.forget();

    // Wait for response
    match receiver.await {
        Ok(response) => response,
        Err(error) => {
            console_log!("Error receiving response: {:?}", error);
            Err(JsValue::from_str(&error.to_string()))
        }
    }
}
