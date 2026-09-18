mod analytics;
mod audience;
mod auth;
mod config;
mod db;
mod distribution;
mod email;
mod generate;
mod payments;
mod posts;
mod publications;
mod push;
mod storage;

use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

/// Pre-Cycle 0 boot: a bare health check on the port the canon doc and
/// BB26090903's docker-compose.yml already agree on (3001), plus
/// BB26091207's push-subscription routes -- the first routes in this
/// service to do real work ahead of Week 2's Postgres wiring (see
/// `push::store`'s doc comment for why an in-memory store is the right
/// call for now). Not yet connected to Postgres/Redis otherwise -- see
/// db.rs's own doc comment for why that's deliberate at this stage, not
/// an oversight.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let push_store = Arc::new(push::store::PushStore::default());

    let app = Router::new()
        .route("/health", get(health))
        .route("/push/subscribe", post(push::handlers::subscribe))
        .route("/push/unsubscribe", post(push::handlers::unsubscribe))
        .with_state(push_store);

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
