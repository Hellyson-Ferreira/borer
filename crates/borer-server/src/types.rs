use std::sync::Arc;
use axum::extract::ws::{Message, WebSocket};
use tokio::sync::Mutex;
use futures::stream::SplitSink;

pub type SharedWs = Arc<Mutex<Option<SplitSink<WebSocket, Message>>>>;