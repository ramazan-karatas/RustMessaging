use axum::{
    extract::{State, Json},
};

use crate::{
    app::AppState,
    infra::error::AppError,
    service::health,
};
use serde::Serialize;


#[derive(Serialize)]
pub struct HealthResponse {
    status: &'static str,
}



pub async fn health(
    State(state): State<AppState>,
) -> Result<Json<HealthResponse>, AppError> {
    health::check_health(&state.db).await?;
    Ok(Json(HealthResponse {
        status: "ok",
    }))
}

