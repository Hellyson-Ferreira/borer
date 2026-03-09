use crate::state::AppState;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use borer_core::protocol::TunnelMessage;
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;

const CHANNEL_BUFFER_SIZE: usize = 256;

pub(crate) async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, state: AppState) {
    let (mut ws_sink, mut ws_stream) = socket.split();
    let (tx, mut rx) = mpsc::channel::<Message>(CHANNEL_BUFFER_SIZE);

    {
        let mut guard = state.ws_tx.lock().await;
        *guard = Some(tx);
    }

    println!("WebSocket client connected");

    let write_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sink.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Binary(bytes)) => {
                match TunnelMessage::from_bytes(&bytes) {
                    Ok(TunnelMessage::HttpResponse(resp)) => {
                        let mut pending = state.pending.lock().await;

                        if let Some(tx) = pending.remove(&resp.id) {
                            let _ = tx.send(resp);
                        } else {
                            println!("No pending request for id: {}", resp.id);
                        }
                    }
                    Ok(other) => {
                        println!("Unexpected tunnel message: {:?}", other);
                    }
                    Err(e) => {
                        println!("Invalid tunnel message: {e}");
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("WebSocket read error: {e}");
                break;
            }
        }
    }

    println!("WebSocket client disconnected");

    write_task.abort();

    let mut guard = state.ws_tx.lock().await;
    *guard = None;
}
