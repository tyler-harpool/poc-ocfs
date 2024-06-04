use axum::{
    extract::Extension,
    routing::{delete, get, patch, post},
    Router,
};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;
mod db;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Set up logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Establish connection to the database using the function from db.rs
    let pool = db::establish_connection().await;

    // Optional: Run migrations
    db::run_migrations(&pool).await;

    // Define the application routes
    let app = Router::new()
        .route("/case_data/:id", get(handlers::get_case_data))
        .route("/case_data", post(handlers::create_case_data))
        .route("/case_data/:id", patch(handlers::update_case_data))
        .route("/case_data/:id", delete(handlers::delete_case_data))
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner());

    // Bind to the address and serve the application
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
