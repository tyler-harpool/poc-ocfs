use database_utils::{establish_connection, run_migrations};

use axum::{
    extract::Extension,
    routing::{delete, get, patch, post},
    Router,
};
use dotenv::dotenv;
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


    match dotenv::var("DATABASE_URL") {
        Ok(val) => println!("DATABASE_URL is set to: {}", val),
        Err(e) => println!("DATABASE_URL is not set: {}", e),
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
        .route("/case_data", get(handlers::list_all_case_data))
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
