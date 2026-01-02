use crate::types::SharedWs;
use axum::body::Bytes;
use axum::extract::{OriginalUri, State, WebSocketUpgrade};
use axum::extract::ws::{Message, WebSocket};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::response::IntoResponse;
use borer_core::protocol::{TunnelHttpRequest, TunnelMessage};
use futures::{SinkExt, StreamExt};


pub async fn proxy(
    State(state): State<SharedWs>,
    method: Method,
    uri: OriginalUri,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let headers_vec = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();

    let req = TunnelHttpRequest {
        method: method.to_string(),
        path: uri.0.path().to_string(),
        query: uri.0.query().map(|q| q.to_string()),
        headers: headers_vec,
        body: body.to_vec(),
    };

    let msg = TunnelMessage::HttpRequest(req);
    let bytes = msg.to_bytes().unwrap();

    let mut sender = {
        let mut guard = state.lock().await;
        guard.take()
    };

    if let Some(ref mut ws) = sender {
        if ws.send(Message::Binary(Bytes::from(bytes))).await.is_ok() {
            let mut guard = state.lock().await;
            *guard = sender;
            return (StatusCode::OK, "sent to client");
        }
    }

    (StatusCode::SERVICE_UNAVAILABLE, "no client connected")
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<SharedWs>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, State(state)))
}

async fn handle_ws(socket: WebSocket, State(state): State<SharedWs>) {
    let (sender, mut receiver) = socket.split();

    {
        let mut guard = state.lock().await;
        *guard = Some(sender);
    }

    println!("WebSocket client connected");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Binary(bytes)) => {
                println!("Received binary message: {} bytes", bytes.len());
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("WS error: {e}");
                break;
            }
        }
    }

    println!("WebSocket client disconnected");

    let mut guard = state.lock().await;
    *guard = None;
}
