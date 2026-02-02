use axum::{
    extract::State,
    routing::{delete, post, get},
    Json, Router,
};

use serde::Serialize;
use sqlx::PgPool;

use crate::{
    api::{messages, users},
    infra::error::AppError,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))

        // POST /create-user -> yeni user olusturur.
        .route("/create-user", post(users::create_user_handler))

        // GET /users -> kullanicilari listeler.
        .route("/users", get(users::list_users_handler))

        // message endpointleri

        // POST message/send-message -> mesaj gonder
        .route("/send-message", post(messages::send_message_handler))

        // GET /users/:id/inbox -> :id'li kullanicinin gelen kutusunu listeler
        .route("/users/{id}/inbox", get(messages::list_inbox_handler))

        // GET /users/:id/sent -> :id'li kullanicinin gonderilenlerini listeler
        .route("/users/{id}/sent", get(messages::list_sent_handler))

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