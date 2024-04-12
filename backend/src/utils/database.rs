use anyhow::{Context, Result};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::info;

pub type ConnectionPool = Pool<Postgres>;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: ConnectionPool,
}

impl Database {
    pub async fn new(database_url: &str, run_migration: bool) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .context("Failed to connect to Postgres")?;

        if run_migration {
            info!("Running database migrations");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .context("Failed to run database migrations")?;

            info!("Database migrations completed");
        }

        Ok(Self { pool })
    }
}


#[cfg(test)]
mod tests {
    use std::env::var;
    use super::*;

    #[tokio::test]
    async fn should_return_pool() {
        dotenv::dotenv().ok();

        let database_url = var("DATABASE_URI").unwrap();
        let database = Database::new(&database_url, false).await;

        assert!(database.is_ok());
    }

    #[tokio::test]
    async fn should_return_error_when_use_invalid_creds() {
        let invalid_database_url = "postgres://localhost:5432";
        let database = Database::new(&invalid_database_url, false).await;

        assert!(database.is_err());
    }
}

