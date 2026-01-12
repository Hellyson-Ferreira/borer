use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use borer_core::protocol::TunnelMessage;
use futures::StreamExt;

pub(crate) async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, state: AppState) {
    let (sender, mut receiver) = socket.split();

    {
        let mut guard = state.ws.lock().await;
        *guard = Some(sender);
    }

    println!("WebSocket client connected");

    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Binary(bytes)) => {
                println!("Received binary message: {} bytes", bytes.len());

                match TunnelMessage::from_bytes(&bytes) {
                    Ok(TunnelMessage::HttpResponse(resp)) => {
                        println!("SERVER RECEIVED RESPONSE: {}", resp.status);

                        let mut pending = state.pending.lock().await;

                        if let Some(tx) = pending.remove(&resp.id) {
                            let _ = tx.send(resp);
                        } else {
                            println!("No pending request for id: {}", resp.id);
                        }
                    }
                    Ok(other) => {
                        println!("SERVER RECEIVED OTHER: {:?}", other);
                    }
                    Err(e) => {
                        println!("SERVER INVALID MESSAGE: {e}");
                    }
                }
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

    let mut guard = state.ws.lock().await;
    *guard = None;
}
