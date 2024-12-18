use axum::{Router, routing::get, middleware::from_fn};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

mod handlers;
mod middleware;

pub struct AppState {
    pub store: async_session::MemoryStore,
    pub oauth_client: oauth2::basic::BasicClient,
    pub http_client: reqwest::Client,
}


pub fn create_router(app_state: Arc<AppState>) -> Router {
    let app_state_for_middleware = app_state.clone();

    Router::new()
        .route("/", get(handlers::root))
        .route("/auth/discord", get(handlers::discord_auth))
        .route("/auth/authorized", get(handlers::login_authorized))
        .route("/logout", get(handlers::logout))
        .route("/adr", get(handlers::forward_adr))
        .route("/attorney_advocates", get(handlers::forward_attorney_advocate))
        .route("/case_data", get(handlers::forward_case_data))
        .route("/charges", get(handlers::forward_charges))
        .route("/civil_judgments", get(handlers::forward_civil_judgments))
        .route("/dependencyPermanency", get(handlers::forward_dependency_permanency))
        .route("/diversions", get(handlers::forward_diversion))
        .route("/hearings_events", get(handlers::forward_hearings_events))
        .route("/motions_filings", get(handlers::forward_motions_filings))
        .route("/orders", get(handlers::forward_orders))
        .route("/participants", get(handlers::forward_participants))
        .route("/pleadings", get(handlers::forward_pleadings))
        .route("/postTrial", get(handlers::forward_post_trial))
        .route("/pretrial_intake", get(handlers::forward_pretrial_intake))
        .route("/probateReviewMonitor", get(handlers::forward_probate))
        .route("/sanctions", get(handlers::forward_sanctions))
        .route("/status", get(handlers::forward_status))
        .route("/ocr", get(handlers::forward_pdf))
        .route("/judges", get(handlers::forward_judges))
        .layer(CorsLayer::new())
        .layer(from_fn(move |req, next| {
            middleware::auth_middleware(app_state_for_middleware.clone(), req, next)
        }))
        .with_state(app_state)


    }