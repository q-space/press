mod analytics;
mod audience;
mod auth;
mod canvas;
mod config;
mod db;
mod distribution;
mod email;
mod generate;
mod payments;
mod posts;
mod publications;
mod storage;

use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

/// Shared app state. `db` is an `Option` rather than a bare
/// `DatabaseConnection` because most routes here still don't need one
/// (see `db.rs`'s doc comment) -- a route that does (Canvas, so far)
/// checks for `None` itself and answers 503 rather than the whole API
/// refusing to boot without Postgres.
#[derive(Clone)]
pub struct AppState {
    pub db: Option<DatabaseConnection>,
}

/// Boot: health check plus whatever's actually wired so far. BB26091502
/// (Canvas) is the first feature in this API with real routes mounted and
/// a real DB connection attempt -- everything else here (posts,
/// publications, generate) is still reachable only by direct fn calls
/// pending its own routing pass (see BB26091205).
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();

    let db = if config.database_url.is_empty() {
        tracing::warn!("DATABASE_URL not set -- routes that need a database will return 503");
        None
    } else {
        match db::connect(&config.database_url).await {
            Ok(conn) => Some(conn),
            Err(err) => {
                tracing::warn!(
                    "could not connect to DATABASE_URL ({err}) -- routes that need a database will return 503"
                );
                None
            }
        }
    };

    let state = AppState { db };

    let app = Router::new()
        .route("/health", get(health))
        .merge(canvas::handlers::routes())
        .merge(canvas::handlers::public_routes())
        .with_state(state);

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
