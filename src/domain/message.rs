use uuid::Uuid;
use chrono::{DateTime,Utc};
pub struct Message {
    pub id: Uuid,
    pub from_user_id: Uuid,
    pub to_user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>, // veritabaniyla uyusmazligin onune gecmek icin bunu kullandik.
    pub read_at: Option<DateTime<Utc>>, // veritabaninda read_at dolu olmak zorunda degil. Cunku mesaj okunmamis olabilir.
}