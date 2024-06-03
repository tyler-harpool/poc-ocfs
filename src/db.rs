use sqlx::{Pool, Postgres, migrate::Migrator};
use std::path::Path;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use tracing::info;
pub async fn establish_connection() -> PgPool {
    info!("Starting database connection");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut retries = 5;
    loop {
        match PgPoolOptions::new()
            .max_connections(10)  // Adjust based on your application needs
            .acquire_timeout(std::time::Duration::from_secs(30))  // Increase connection timeout
            .connect(&database_url)
            .await {
            Ok(pool) => return pool,
            Err(e) => {
                if retries > 0 {
                    retries -= 1;
                    info!("Retrying to connect to the database. Retries left: {}", retries);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                } else {
                    panic!("Failed to connect to the database: {:?}", e);
                }
            }
        }
    }
}

pub async fn run_migrations(pool: &Pool<Postgres>) {
    let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();
    migrator.run(pool).await.unwrap();
}
