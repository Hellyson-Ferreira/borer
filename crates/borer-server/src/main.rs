mod auth;
mod router;
mod server;
mod session;
mod state;

use crate::router::{index, proxy, ws_handler};
use crate::state::AppState;
use axum::Router;
use axum::routing::{any, get};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let state = AppState {
        ws: Arc::new(Mutex::new(None)),
        pending: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(ws_handler))
        .route("/proxy/{*path}", any(proxy))
        .route("/proxy", any(proxy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
