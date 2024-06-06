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
        .route("/", get(|| async { "Welcome to the Pleadings API!" }))
        .route("/pleadings", get(handlers::list_all_pleadings))
        .route("/pleadings/:id", get(handlers::get_pleading))
        .route("/pleadings", post(handlers::create_pleading))
        .route("/pleadings/:id", patch(handlers::update_pleading))
        .route("/pleadings/:id", delete(handlers::delete_pleading))
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner())
}

pub async fn start_server(app: Router) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3011));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
