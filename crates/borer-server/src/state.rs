use axum::extract::ws::{Message, WebSocket};
use borer_core::protocol::TunnelHttpResponse;
use futures::stream::SplitSink;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, oneshot};

pub type SharedWs = Arc<Mutex<Option<SplitSink<WebSocket, Message>>>>;
pub type PendingRequests = Arc<Mutex<HashMap<String, oneshot::Sender<TunnelHttpResponse>>>>;

#[derive(Clone)]
pub struct AppState {
    pub ws: SharedWs,
    pub pending: PendingRequests,
}
