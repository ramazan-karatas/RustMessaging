use axum::{
    routing::{ post, get },
    Router,
};

use sqlx::PgPool;

use crate::{
    api::{messages, users, health},
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health)) // /health endpointine get istegi gonderildiginde get(health) calistirir.

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
