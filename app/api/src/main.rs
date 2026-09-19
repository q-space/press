mod analytics;
mod audience;
mod auth;
mod config;
mod db;
mod distribution;
mod email;
mod generate;
mod lists;
mod payments;
mod posts;
mod publications;
mod storage;

use axum::{routing::get, Router};
use std::sync::Arc;

/// Pre-Cycle 0 boot: a bare health check on the port the canon doc and
/// BB26090903's docker-compose.yml already agree on (3001). Not yet
/// connected to Postgres/Redis -- see db.rs's own doc comment for why
/// that's deliberate at this stage, not an oversight.
///
/// BB26091208 adds the generation engine's first real HTTP surface
/// (`generate::routes::router`) -- schema/preview/generate plus the
/// field-level AI-assist endpoints. See that module's own doc comment for
/// why it's a deliberately narrow slice, not the full BB26091205 API.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let generate_state = generate::routes::GenerateState {
        client: reqwest::Client::new(),
        groq_api_key: config.groq_api_key.clone(),
        doc_registry: Arc::new(generate::doc_registry::DocRegistry::from_env()),
    };

    let app = Router::new()
        .route("/health", get(health))
        .merge(generate::routes::router(generate_state));

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
