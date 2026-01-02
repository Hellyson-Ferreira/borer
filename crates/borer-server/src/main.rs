mod auth;
mod router;
mod server;
mod session;
mod types;

use crate::router::{proxy, ws_handler};
use crate::types::SharedWs;
use axum::routing::{any, get};
use axum::{Router};
use std::sync::Arc;

use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let ws_state: SharedWs = Arc::new(Mutex::new(None));

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/proxy", any(proxy))
        .with_state(ws_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
