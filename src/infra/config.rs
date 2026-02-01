use std::env;
use dotenvy::dotenv;
use std::path::Path;
use crate::infra::error::AppError;

pub struct AppConfig {
    pub database_url: String,
    pub server_addr: String,
}
impl AppConfig{
    pub(crate) fn from_env() -> Result<Self, AppError>{
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").map_err(|_|
        AppError::MissingEnv("DATABASE_URL".to_string()))?;

        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_|
        "127.0.0.1:8000".to_string());

        Ok(AppConfig  { database_url, server_addr })
    }
}