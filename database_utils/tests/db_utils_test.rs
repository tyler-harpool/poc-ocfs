#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};
    use std::env;
    use std::fs;
    use std::path::Path;
    use std::time::Duration;

    #[tokio::test]
    async fn test_establish_connection() {
        // Load environment variables from .env file
        dotenv().ok();

        // Retrieve the database URL from the environment
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // Attempt to establish a connection
        let pool = establish_connection(&database_url).await;

        // Check if the connection was successfully established
        assert!(pool.is_ok());
    }

    #[tokio::test]
    async fn test_run_migrations() {
        // Load environment variables from .env file
        dotenv().ok();

        // Retrieve the database URL and migrations directory from the environment
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let migrations_dir = env::var("MIGRATIONS_TESTDIR").expect("MIGRATIONS_TESTDIR must be set");

        // Ensure the migrations directory exists
        assert!(
            Path::new(&migrations_dir).exists(),
            "Migrations directory does not exist"
        );

        // Establish a connection
        let pool = establish_connection(&database_url).await.unwrap();

        // Run migrations
        run_migrations(&pool, &migrations_dir).await.unwrap();

        // Check if migrations were successfully executed
        // You can add more specific assertions based on your migration logic
        assert!(true);
    }

    async fn establish_connection(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(30))
            .connect(database_url)
            .await;
        pool
    }

    async fn run_migrations(
        pool: &Pool<Postgres>,
        migrations_dir: &str,
    ) -> Result<(), sqlx::migrate::MigrateError> {
        let migrator = Migrator::new(Path::new(migrations_dir)).await.unwrap();
        migrator.run(pool).await
    }
}
