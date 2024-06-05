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
        .route("/attorney_advocates", get(handlers::list_all_attorney_advocates))
        .route("/attorney_advocates/:id", get(handlers::get_attorney_advocate))
        .route("/attorney_advocates", post(handlers::create_attorney_advocate))
        .route("/attorney_advocates/:id", patch(handlers::update_attorney_advocate))
        .route("/attorney_advocates/:id", delete(handlers::delete_attorney_advocate))
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner())
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
