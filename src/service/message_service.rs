// burada message service isleri olacak
// send_message, inbox ve sent endpointlerini ilgilendiren service fonksiyonlari burada olacak.


use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::message::Message;
use crate::infra::error::AppError;
use crate::repo::message_repo;

pub async fn send_message_service(
    pool: &PgPool,
    from_user_id: Uuid,
    to_user_id: Uuid,
    content:String, )
    -> Result<Message, AppError>{
    let normalized_content = content.trim(); // message icerigi normalize ediliyor.

    if normalized_content.is_empty() { // mesaj icerigi bos mu kontrolu yapiliyor.
        return Err(AppError::EmptyMessage( // EmptyMessage hatasi donduruluyor.
            "Message content cannot be empty".to_string(),
        )) ;
    }

    let message = message_repo::send_message(&pool, from_user_id, to_user_id, normalized_content).await?; // mesajin bos olmadigi senaryoda bu send_message fonksiyonu cagiriliyor
    // Bu fonksiyon Result<Message, sqlx::Error> donuyor, Ok donerse message return ediliyor.
    Ok(message)
}

pub async fn list_inbox_service(pool: &PgPool, user_id: Uuid)
    -> Result<Vec<Message>, AppError> {
    let messages = message_repo::list_inbox(&pool, user_id).await?;
    Ok(messages)
}


pub async fn list_sent_service(pool: &PgPool, user_id: Uuid) -> Result<Vec<Message>, AppError> {
    let messages = message_repo::list_sent(pool, user_id).await?;
    Ok(messages)
}