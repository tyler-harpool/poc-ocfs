use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use axum::{
    extract::Extension,
    routing::{delete, get, patch, post},
    Router,
};

use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

// Declare the handlers module
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
        .route("/case_data", get(handlers::list_all_case_data))
        .route("/case_data/:id", get(handlers::get_case_data))
        .route("/case_data", post(handlers::create_case_data))
        .route("/case_data/:id", patch(handlers::update_case_data))
        .route("/case_data/:id", delete(handlers::delete_case_data))
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner())
        .layer(CorsLayer::permissive())
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
