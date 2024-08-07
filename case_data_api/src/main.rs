use database_utils::{establish_connection, run_migrations};

use dotenv::dotenv;

use ::case_data_api::{create_app, setup_logging, start_server};
pub mod handlers;
pub mod models;

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
