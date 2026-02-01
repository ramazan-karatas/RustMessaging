
use crate::domain::user::User;
use sqlx;

use uuid::Uuid;

async fn create_user(pool: &sqlx::PgPool, username: &str) -> Result<User, sqlx::Error> { // creates user
    let id = Uuid::new_v4();
    let simdi = chrono::Utc::now();

    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, created_at)
        VALUES ($1, $2, $3)
        RETURNING id, username, created_at
        "#,
        id,
        username,
        simdi,
    )
        .fetch_one(pool)
        .await

}

async fn find_by_id(pool: &sqlx::PgPool, id: uuid::Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
        .fetch_one(pool)
        .await
}
async fn get_all_users(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, created_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
        .fetch_all(pool)
        .await
}