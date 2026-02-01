use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}