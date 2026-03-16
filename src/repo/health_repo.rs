use sqlx::PgPool;

use crate::infra::error::AppError;

pub async fn ping_db(pool: &PgPool) -> Result<(), AppError> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    Ok(())
}