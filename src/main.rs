mod service;
mod repo;
mod domain;
mod infra;
mod app;
mod api;

use std::net::SocketAddr;

use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

use crate::infra::{config::AppConfig, db::create_pool};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info,tower_http=info")),
        )
        .init();

    let config = match AppConfig::from_env() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to load config: {}", e);
            std::process::exit(1);
        }
    };

    let pool = match create_pool(config.database_url.as_str()).await {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("Failed to connect DB: {}", e);
            std::process::exit(1);
        }
    };

    let state = app::AppState { db: pool };

    let app = app::build_router(state).layer(TraceLayer::new_for_http());

    let addr: SocketAddr = config
        .server_addr
        .parse()
        .expect("SERVER_ADDR must be like 127.0.0.1:8000");

    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}