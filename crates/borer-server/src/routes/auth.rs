use crate::routes::utils::get_token_from_header;
use crate::state::AppState;
use axum::response::IntoResponse;
use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    pub name: Option<String>,
}

pub async fn create_token(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateTokenRequest>,
) -> impl IntoResponse {
    let auth_header = get_token_from_header(&headers);

    let Some(token) = auth_header else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if !state.auth.validate_master(&token) {
        return Err(StatusCode::FORBIDDEN);
    }

    let token = state.auth.create_token(payload.name).await;

    match token {
        Ok(t) => Ok(Json(t).into_response()),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn validate(State(state): State<AppState>, headers: HeaderMap) -> StatusCode {
    let auth_header = get_token_from_header(&headers);

    let Some(token) = auth_header else {
        return StatusCode::UNAUTHORIZED;
    };

    match state.auth.validate_token(&token).await {
        Ok(Some(_)) => StatusCode::OK,
        Ok(None) => StatusCode::UNAUTHORIZED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
