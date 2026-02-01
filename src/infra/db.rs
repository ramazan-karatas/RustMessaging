use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::infra::error::AppError;

pub async fn create_pool(database_url: &str) -> Result<PgPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(database_url)
        .await?;

    Ok(pool)
}