use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    response::IntoResponse
};

use crate::{app::AppState,
        repo::user_repo,
        domain::user::User,
        infra::error::AppError,
};

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
}

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Result<impl IntoResponse, AppError>
{
    let user = user_repo::create_user(&state.db, &payload.username.as_str()).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user_handler(
    State(state): State<AppState>,
    Path(id): Path<uuid::Uuid>, // urldeki ID'yi id degiskenine aliyoruz.
) -> Result<impl IntoResponse, AppError> {
    let user = user_repo::find_by_id(&state.db, id).await?;

    Ok(Json(user))
}

pub async fn list_users_handler (
    State(state): State<AppState>
) -> Result<impl IntoResponse, AppError> {
    let users = user_repo::get_all_users(&state.db).await?;
    Ok(Json(users)) // Vec<User> yapisini Json ile paketledik.
}