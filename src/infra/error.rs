use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody{
    error: String,
}
#[derive(Debug, Error)]
pub enum AppError{
    #[error("Missing Env")]
    MissingEnv(String),
    #[error("DB Error")]
    Db(#[from] sqlx::Error),
    #[error("Bad Request")]
    BadRequest(String),
    #[error("Not Found")]
    NotFound(String),
    #[error("Conflict")]
    Conflict(String),
    #[error("Internal Error")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),

            AppError::MissingEnv(var) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("missing env variable: {}", var),
                ),
            AppError::Db(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database error".to_string(),
                ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
                )
        };

        let body = Json(ErrorBody { error: message });
        (status, body).into_response()
    }
}