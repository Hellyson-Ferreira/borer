use axum::body::Body;
use std::time::Duration;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::{OriginalUri, State, WebSocketUpgrade};
use axum::http::{HeaderMap, Method, Response, StatusCode};
use axum::response::IntoResponse;
use borer_core::protocol::{TunnelHttpRequest, TunnelMessage};
use futures::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::AppState;
use bytes::Bytes;
use tokio::sync::oneshot;
use tokio::time::timeout;

pub async fn proxy(
    State(state): State<AppState>,
    method: Method,
    uri: OriginalUri,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let headers_vec = headers
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<Vec<_>>();

    let path = uri.0.path()[6..].to_string();

    let id = Uuid::new_v4().to_string();

    let req = TunnelHttpRequest {
        id: id.clone(),
        method: method.to_string(),
        path,
        query: uri.0.query().map(|q| q.to_string()),
        headers: headers_vec,
        body: body.to_vec(),
    };

    let msg = TunnelMessage::HttpRequest(req);
    let bytes = match msg.to_bytes() {
        Ok(b) => b,
        Err(_) => {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let (tx, rx) = oneshot::channel();

    {
        let mut map = state.pending.lock().await;
        map.insert(id.clone(), tx);
    }

    let mut sender = {
        let mut guard = state.ws.lock().await;
        guard.take()
    };

    if let Some(ref mut ws) = sender {
        if ws.send(Message::Binary(Bytes::from(bytes))).await.is_ok() {
            let mut guard = state.ws.lock().await;
            *guard = sender;
        } else {
            state.cleanup_pending(&id).await;
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    } else {
        state.cleanup_pending(&id).await;
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }

    let resp = match timeout(Duration::from_secs(30), rx).await {
        Ok(Ok(resp)) => resp,
        _ => {
            state.cleanup_pending(&id).await;
            return StatusCode::GATEWAY_TIMEOUT.into_response();
        }
    };

    let mut response = Response::builder().status(resp.status);

    for (k, v) in resp.headers {
        response = response.header(k, v);
    }

    response
        .body(Body::from(resp.body))
        .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
}

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
