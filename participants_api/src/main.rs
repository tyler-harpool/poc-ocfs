use database_utils::{establish_connection, run_migrations};

use axum::{
    extract::Extension,
    routing::{delete, get, patch, post},
    Router,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Debug statement to verify environment variable loading
    match dotenv::var("DATABASE_URL") {
        Ok(val) => println!("DATABASE_URL is set to: {}", val),
        Err(e) => println!("DATABASE_URL is not set: {}", e),
    }

    match dotenv::var("MIGRATIONS_DIR") {
        Ok(val) => println!("MIGRATIONS_DIR is set to: {}", val),
        Err(e) => println!("MIGRATIONS_DIR is not set: {}", e),
    }

    // Set up logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Establish database connection
    let pool = establish_connection().await;

    // Run migrations
    run_migrations(&pool).await;

    // Define the application routes
    let app = Router::new()
        .route("/", get(|| async { "Welcome to OCFS!" }))
        .route("/participants", get(handlers::list_all_participants))
        .route("/participants/:id", get(handlers::get_participant))
        .route("/participants", post(handlers::create_participant))
        .route("/participants/:id", patch(handlers::update_participant))
        .route("/participants/:id", delete(handlers::delete_participant))
        .layer(ServiceBuilder::new().layer(Extension(pool)).into_inner());

    // Bind to the address and serve the application
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    println!("Listening on: {:?}, {:?}", listener.local_addr().unwrap(), port);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
