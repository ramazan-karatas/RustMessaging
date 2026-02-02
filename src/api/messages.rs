use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};

use uuid::Uuid;

use crate::{
    app::AppState,
    infra::error::AppError,
    repo::message_repo,
};


#[derive(serde::Deserialize)]
pub struct SendMessageRequest { // mesaj atarken kullanicidan beklenen bilgiler.
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub content: String,
}

// mesaj gonderme: POST /messages
pub async fn send_message_handler(
    State(state): State<AppState>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<impl IntoResponse, AppError> {
    let message = message_repo::send_message(
        &state.db,
        payload.from_user_id,
        payload.to_user_id,
        &payload.content.as_str(),
    ).await?;
    Ok((StatusCode::CREATED, Json(message)))
}

// Inbox Handler
// GET /users/:id/inbox
pub async fn list_inbox_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<impl IntoResponse, AppError> {
    let messages = message_repo::list_inbox(&state.db, user_id).await?;
    Ok(Json(messages))
}

// SentBox Handler
// GET /users/:id/sent
pub async fn list_sent_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<impl IntoResponse, AppError>
{
    let messages = message_repo::list_sent(&state.db, user_id).await?;
    Ok(Json(messages))
}