use axum::{
    extract::State,
    routing::get,
    Json, Router,
};

use serde::Serialize;
use sqlx::PgPool;

use crate::infra::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(state)
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    sqlx::query("SELECT 1").execute(&state.db).await?;
    Ok(Json(HealthResponse { status: "ok" }))
}