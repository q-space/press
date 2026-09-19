//! The binary entrypoint -- module declarations live in `lib.rs` now
//! (BB26091205) so `src/bin/mint_token.rs` can reuse them too. `lists`
//! (merged from `dev`) is declared there too -- see `lib.rs`.

use axum::{routing::get, Json, Router};
use qspace_press_api::{config, generate};
use serde_json::{json, Value};

/// Pre-Cycle 0 boot: a bare health check on the port the canon doc and
/// BB26090903's docker-compose.yml already agree on (3001). Not yet
/// connected to Postgres/Redis -- see db.rs's own doc comment for why
/// that's deliberate at this stage, not an oversight.
///
/// BB26091205 adds the first real routes: `generate::handlers::router`
/// carries its own `Config` state (the thin-client auth extractor needs
/// `jwt_secret`), so it's merged in rather than sharing the bare
/// `Router::new()` above -- `.merge()` lets a stateless router and an
/// already-`.with_state()`'d one coexist without forcing every other
/// route in this file to carry state it doesn't need yet.
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let app = Router::new()
        .route("/health", get(health))
        // BB26091205: iSconl's `hub` (lib/engine-client.js) expects every
        // engine it talks to -- Qpress now included -- to answer
        // `{"status":"ok"}` (not plain text) on /health and 200 JSON on
        // /manifest, the shape every JS engine's own boot already speaks.
        // Qpress doesn't declare capabilities through this yet (the 4
        // thin-client routes are wired directly in hub's api-compat.js,
        // not through hub's dynamic capability registry -- see that
        // repo's own BB26091205 commit for why), so an empty list here is
        // correct today, not a placeholder for a TODO.
        .route("/manifest", get(manifest))
        .merge(generate::handlers::router(config.clone()));

    let addr = format!("0.0.0.0:{}", config.port);
    tracing::info!("qspace-press-api listening on {addr}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn manifest() -> Json<Value> {
    Json(json!({ "capabilities": [] }))
}
