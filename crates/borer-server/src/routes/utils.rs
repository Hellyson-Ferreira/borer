use axum::http::HeaderMap;

pub fn get_token_from_header(headers: &HeaderMap) -> Option<String> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    auth_header.map(|s| s.to_string())
}
