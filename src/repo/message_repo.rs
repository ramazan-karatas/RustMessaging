use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::message::Message;

pub async fn send_message(pool: &PgPool, from_user_id: Uuid, to_user_id: Uuid, content: &str)
    -> Result<Message, sqlx::Error>
{
    let id = Uuid::new_v4();
    let now = chrono::Utc::now();
    sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages ( id, from_user_id, to_user_id, content, created_at, read_at )
        VALUES ($1, $2, $3, $4, $5, NULL)
        RETURNING id, from_user_id, to_user_id, content, created_at, read_at
        "#,
        id,
        from_user_id,
        to_user_id,
        content,
        now
    )
        .fetch_one(pool)
        .await
}

pub async fn list_inbox(pool: &PgPool, user_id: Uuid)
    -> Result<Vec<Message>, sqlx::Error>
{
    sqlx::query_as!(
        Message,
        r#"
        SELECT id, from_user_id, to_user_id, content, created_at, read_at
        FROM messages
        WHERE to_user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
        .fetch_all(pool)
        .await
}

pub async fn list_sent(pool: &PgPool, user_id: Uuid)
    -> Result<Vec<Message>, sqlx::Error>
{
    sqlx::query_as!(
        Message,
        r#"
        SELECT id, from_user_id, to_user_id, content, created_at, read_at
        FROM messages
        WHERE from_user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
        .fetch_all(pool)
        .await
}