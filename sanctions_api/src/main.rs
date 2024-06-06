use database_utils::{establish_connection, run_migrations};
use dotenv::dotenv;
use sanctions_api::{create_app, setup_logging, start_server};

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logging();

    // Establish database connection
    let pool = establish_connection().await;

    // Run migrations
    run_migrations(&pool).await;

    // Create the application router
    let app = create_app(pool);

    // Bind to the address and serve the application
    start_server(app).await;
}
