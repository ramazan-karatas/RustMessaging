use uuid::Uuid;
use chrono::{DateTime,Utc};
struct Message {
    id: Uuid,
    from_user_id: Uuid,
    to_user_id: Uuid,
    content: String,
    created_at: DateTime<Utc>, // veritabaniyla uyusmazligin onune gecmek icin bunu kullandik.
    read_at: Option<DateTime<Utc>>, // veritabaninda read_at dolu olmak zorunda degil. Cunku mesaj okunmamis olabilir.
}