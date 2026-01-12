use crate::state::AppState;
use axum::body::Body;
use axum::extract::ws::Message;
use axum::extract::{OriginalUri, State};
use axum::http::{HeaderMap, Method, Response, StatusCode};
use axum::response::{Html, IntoResponse};
use borer_core::protocol::{TunnelHttpRequest, TunnelMessage};
use bytes::Bytes;
use futures::SinkExt;
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::timeout;
use uuid::Uuid;

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

pub async fn index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}
