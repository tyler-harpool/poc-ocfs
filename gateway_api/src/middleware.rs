use axum::{
    body::Body,
    http::Request,
    response::Response,
};
use std::sync::Arc;
use crate::AppState;
use async_session::SessionStore;

pub async fn auth_middleware(
    state: Arc<AppState>,
    mut req: Request<Body>,
    next: axum::middleware::Next,
) -> Result<Response, (axum::http::StatusCode, String)> {
    let session_cookie = req
        .headers()
        .get("Cookie")
        .and_then(|cookie| cookie.to_str().ok())
        .and_then(|cookie_str| cookie_str.split(';').find(|c| c.trim().starts_with("session=")));

    if let Some(session_cookie) = session_cookie {
        let session_id = session_cookie.trim_start_matches("session=");
        if let Ok(Some(session)) = state.store.load_session(session_id.to_string()).await {
            req.extensions_mut().insert(session);
        }
    }

    Ok(next.run(req).await)
}