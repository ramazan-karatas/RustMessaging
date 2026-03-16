use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse
};

use crate::{
    app::AppState,
    service::user_service,
    infra::error::AppError,
};

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

pub async fn create_user_handler( // create-user istegi gelmesi durumunda bu fonksiyon calistiriliyor.
    State(state): State<AppState>, // app state'ini aliyor
    Json(payload): Json<CreateUserRequest> // Istek govdesindeki paylaodi aliyor..
) -> Result<impl IntoResponse, AppError> // Response veya AppError donuyor.
{
    let user = user_service::create_user(&state.db, payload.username.to_string()).await?; // user_service'tan create_user fonksiyonunu calistiriyor.
                                                                                        // bu fonksiyon, User veya AppError donuyor.
    Ok((StatusCode::CREATED, Json(user))) // create_user fonksiyonunun User donmesi durumunda CREATED status code'u ve user donduruluyor.
}


pub async fn list_users_handler (
    State(state): State<AppState>
) -> Result<impl IntoResponse, AppError> { // Response veya AppError donduruyor.

    let users = user_service::list_all_users(&state.db).await?; // user_service icindeki list_all_users fonksiyonu cagiriliyor. Bu fonksiyon Result<Vec<User>, AppError> donduruyor.

    Ok(Json(users)) // Vec<User> yapisini Json ile paketledik.
}