// Service katmanının user service kısmı,
// burada create user, list users bulunacak.

use sqlx::PgPool;
use crate::{
    domain::user::User,
    infra::error::AppError,
    repo::user_repo,
};
pub async fn create_user(pool: &PgPool, username: String)  // pool'u ve username'i aliyor.
    -> Result<User, AppError> {  // User veya AppError donuyor

    let normalized_username = username.trim(); // username normalize ediliyor.

    if normalized_username.is_empty() { // Username bos mu kontrolu yapiliyor.
        return Err(AppError::Validation( // username bos ise ilgili hata metni donduruluyor.
            "Username cannot be empty".to_string(),
        ));
    }

    // username bos degilse user_repo'dan create_user fonksiyonu cagiriliyor.
    let user = user_repo::create_user(pool, normalized_username).await?;  // bu fonksiyon Result<User, sqlx::Error> donuyor.
    Ok(user) // user olustuysa bu deger donduruluyor.
}

pub async fn list_all_users(pool: &PgPool) -> Result<Vec<User>, AppError> {

    let users = user_repo::get_all_users(pool).await?; // user_repo'dan get_all_users fonksiyonu cagiriliyor. Bu fonksiyon Result<Vec<User>, sqlx::Error> donduruyor.

    Ok(users) // get_all_users'in error dondurmedigi senaryoda users vectoru donduruluyor.
}
