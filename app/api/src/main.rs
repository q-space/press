mod analytics;
mod audience;
mod auth;
mod config;
mod db;
mod distribution;
mod email;
mod payments;
mod posts;
mod publications;
mod storage;

use axum::{routing::get, Router};

/// Pre-Cycle 0 boot: a bare health check on the port the canon doc and
/// BB26090903's docker-compose.yml already agree on (3001). Not yet
/// connected to Postgres/Redis -- see db.rs's own doc comment for why
/// that's deliberate at this stage, not an oversight.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let app = Router::new().route("/health", get(health));

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("qspace-press-api listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}

async fn health() -> &'static str {
    "ok"
}
