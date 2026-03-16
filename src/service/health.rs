use sqlx::PgPool;
use crate::{
    infra::error::AppError,
    repo::health_repo,
};

pub async fn check_health(pool: &PgPool) -> Result<(), AppError>{
    health_repo::ping_db(pool).await?;
    Ok(())
}