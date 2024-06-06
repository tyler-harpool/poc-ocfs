
use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
};

use tokio::net::TcpListener;

use tracing_subscriber::EnvFilter;

pub mod handlers;

pub fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn create_app() -> Router {
    Router::new()
        .route("/", get(handlers::forward_adr))
        .route("/api/adr", get(handlers::forward_attorney_adr))
        .route("/api/attorney_advocate", get(handlers::forward_attorney_advocate))
        .route("/api/case_data", get(handlers::forward_case_data))
        .route("/api/charges", get(handlers::forward_charges))
        .route("/api/civil_judgments", get(handlers::forward_civil_judgments))
        .route(
            "/api/dependency_permanency",
            get(handlers::forward_dependency_permanency),
        )
        .route("/api/diversion", get(handlers::forward_diversion))
        .route("/api/hearings_events", get(handlers::forward_hearings_events))
        .route("/api/motions_filings", get(handlers::forward_motions_filings))
        .route("/api/orders", get(handlers::forward_orders))
        .route("/api/participants", get(handlers::forward_participants))
        .route("/api/pleadings", get(handlers::forward_pleadings))
        .route("/api/post_trial", get(handlers::forward_post_trial))
        .route("/api/pretrial_intake", get(handlers::forward_pretrial_intake))
        .route("/api/probate", get(handlers::forward_probate))
        .route("/api/sanctions", get(handlers::forward_sanctions))
        .route("/api/status", get(handlers::forward_status))
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
