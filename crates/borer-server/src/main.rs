mod auth;
mod config;
mod routes;
mod state;
use crate::routes::auth::{create_token, validate};
use crate::routes::proxy::{index, proxy};
use crate::routes::ws::ws_handler;
use crate::state::AppState;
use axum::Router;
use axum::routing::{any, get, post};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = config::load_config();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    let state = AppState {
        ws: Arc::new(Mutex::new(None)),
        pending: Arc::new(Mutex::new(HashMap::new())),
        auth: auth::service::AuthService::new(pool, config.master_token),
    };

    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(ws_handler))
        .route("/auth/validate", post(validate))
        .route("/auth/token", post(create_token))
        .route("/proxy/{*path}", any(proxy))
        .route("/proxy", any(proxy))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
