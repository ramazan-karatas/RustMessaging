use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse,
};

use uuid::Uuid;

use crate::{
    app::AppState,
    service::message_service,
    infra::error::AppError,
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

    // message_service'tan send_message_service() fonksiyonu cagiriliyor. Bu fonksiyon Result<Message, AppError> donuyor.
    let message = message_service::send_message_service(&state.db, payload.from_user_id, payload.to_user_id, payload.content).await?;

    Ok((StatusCode::CREATED, Json(message))) // Mesaj gondermenin ok olmasi durumunda StatusCode::CREATED ve Json(message) donduruluyor.
}

// Inbox Handler
// GET /users/:id/inbox
pub async fn list_inbox_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<impl IntoResponse, AppError> {
    let messages = message_service::list_inbox_service(&state.db, user_id).await?;
    Ok(Json(messages))
}

// SentBox Handler
// GET /users/:id/sent
pub async fn list_sent_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<impl IntoResponse, AppError>
{
    let messages = message_service::list_sent_service(&state.db, user_id).await?;
    Ok(Json(messages))
}