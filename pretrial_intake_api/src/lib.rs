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
        .route("/pretrial_intake", get(handlers::list_all_pretrial_intakes))
        .route("/pretrial_intake/:id", get(handlers::get_pretrial_intake))
        .route("/pretrial_intake", post(handlers::create_pretrial_intake))
        .route(
            "/pretrial_intake/:id",
            patch(handlers::update_pretrial_intake),
        )
        .route(
            "/pretrial_intake/:id",
            delete(handlers::delete_pretrial_intake),
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
