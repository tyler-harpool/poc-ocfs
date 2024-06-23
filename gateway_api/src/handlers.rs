use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
    Json,
};
use serde_json::Value;
use std::sync::Arc;
use crate::AppState;
use oauth2::{CsrfToken, Scope, AuthorizationCode};
use axum::http::StatusCode;

pub async fn root() -> &'static str {
    "Welcome to the API Gateway"
}

pub async fn discord_auth(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

pub async fn login_authorized(
    State(state): State<Arc<AppState>>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    let code = params
        .get("code")
        .ok_or(StatusCode::BAD_REQUEST)?;

    let _token = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(code.to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Here you would typically create a session, store user info, etc.
    // For this example, we'll just redirect to the root
    Ok(Redirect::to("/"))
}

pub async fn logout(_: State<Arc<AppState>>) -> impl IntoResponse {
    // Here you would typically destroy the session
    // For this example, we'll just redirect to the root
    Redirect::to("/")
}


async fn forward_request(
    state: &Arc<AppState>,
    path: &str,
    port: u16
) -> Result<Json<Value>, StatusCode> {
    let url = format!("http://localhost:{}{}", port, path);
    let res = state.http_client
        .get(&url)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    if res.status().is_success() {
        let json = res
            .json::<Value>()
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?;
        Ok(Json(json))
    } else {
        Err(StatusCode::BAD_GATEWAY)
    }
}

pub async fn forward_adr(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/adr", 3000).await
}

pub async fn forward_attorney_advocate(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/attorney_advocates", 3001).await
}

pub async fn forward_case_data(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/case_data", 3002).await
}

pub async fn forward_charges(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/charges", 3003).await
}

pub async fn forward_civil_judgments(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/civil_judgments", 3004).await
}

pub async fn forward_dependency_permanency(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/dependencyPermanency", 3005).await
}

pub async fn forward_diversion(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/diversions", 3006).await
}

pub async fn forward_hearings_events(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/hearings_events", 3007).await
}

pub async fn forward_motions_filings(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/motions_filings", 3008).await
}

pub async fn forward_orders(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/orders", 3009).await
}

pub async fn forward_participants(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/participants", 3010).await
}

pub async fn forward_pleadings(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/pleadings", 3011).await
}

pub async fn forward_post_trial(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/postTrial", 3012).await
}

pub async fn forward_pretrial_intake(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/pretrial_intake", 3013).await
}

pub async fn forward_probate(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/probateReviewMonitor", 3014).await
}

pub async fn forward_sanctions(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/sanctions", 3015).await
}

pub async fn forward_status(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/status", 3016).await
}

pub async fn forward_pdf(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/ocr", 3017).await
}

pub async fn forward_judges(State(state): State<Arc<AppState>>) -> Result<Json<Value>, StatusCode> {
    forward_request(&state, "/judges", 3018).await
}