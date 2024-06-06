use sqlx::PgPool;
use std::net::SocketAddr;

use axum::{
    extract::Extension,
    routing::{delete, get, patch, post},
    Router,
};

use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

pub mod handlers;
pub mod models;

pub fn setup_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

pub fn create_app(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(|| async { "Welcome to OCFS!" }))
        .route(
            "/probateReviewMonitor",
            get(handlers::list_all_probate_review_monitors),
        )
        .route(
            "/probateReviewMonitor/:id",
            get(handlers::get_probate_review_monitor),
        )
        .route(
            "/probateReviewMonitor",
            post(handlers::create_probate_review_monitor),
        )
        .route(
            "/probateReviewMonitor/:id",
            patch(handlers::update_probate_review_monitor),
        )
        .route(
            "/probateReviewMonitor/:id",
            delete(handlers::delete_probate_review_monitor),
        )
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner())
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
