use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tracing::info;

pub async fn connect() -> anyhow::Result<PgPool> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:123456@localhost:5432/beep_rust".to_string());

    info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // Test the connection
    sqlx::query("SELECT 1").execute(&pool).await?;
    
    info!("Database connection established successfully");

    Ok(pool)
}