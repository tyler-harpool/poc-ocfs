
use dotenv::dotenv;
use gateway_api::{create_app, setup_logging, start_server};

#[tokio::main]
async fn main() {
    dotenv().ok();
    setup_logging();

    // Establish database connection


    // Create the application router
    let app = create_app();

    // Bind to the address and serve the application
    start_server(app).await;
}
