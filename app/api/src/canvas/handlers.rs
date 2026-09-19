//! Canvas HTTP surface.
//!
//! `routes()` is the creator-side CRUD (create, fetch, live-edit) that the
//! Next app proxies to (see `app/web/app/api/canvas/**`, which follows the
//! same server-side-proxy shape as `/api/archetypes/*`).
//!
//! `public_routes()` is the unauthenticated share surface -- generic
//! sibling of iSconl's `/api/public/learn/:course/:slug` (built for
//! standalone module export): no login, no account, just the id in the
//! URL. It serves the canvas's own HTML directly rather than wrapping it
//! in JSON, since the artifact is already the self-contained document
//! BB26091204's print rules require (inline `@page`/print styles), so the
//! share link is directly viewable, printable, and embeddable as-is.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use uuid::Uuid;

use super::service::{self, CreateCanvas, UpdateCanvas};
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/canvas", post(create_canvas))
        .route("/canvas/:id", get(get_canvas).put(update_canvas))
}

pub fn public_routes() -> Router<AppState> {
    Router::new().route("/public/canvas/:id", get(public_canvas))
}

async fn create_canvas(
    State(state): State<AppState>,
    Json(input): Json<CreateCanvas>,
) -> Response {
    let Some(db) = &state.db else {
        return db_unavailable();
    };
    if input.title.trim().is_empty() {
        return bad_request("title is required");
    }
    if input.html.trim().is_empty() {
        return bad_request("html is required");
    }
    match service::create(db, input).await {
        Ok(canvas) => (StatusCode::CREATED, Json(canvas)).into_response(),
        Err(err) => server_error(err),
    }
}

async fn get_canvas(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    let Some(db) = &state.db else {
        return db_unavailable();
    };
    match service::get(db, id).await {
        Ok(Some(canvas)) => Json(canvas).into_response(),
        Ok(None) => not_found(),
        Err(err) => server_error(err),
    }
}

async fn update_canvas(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCanvas>,
) -> Response {
    let Some(db) = &state.db else {
        return db_unavailable();
    };
    if input.title.is_none() && input.html.is_none() {
        return bad_request("nothing to update -- send title and/or html");
    }
    match service::update(db, id, input).await {
        Ok(Some(canvas)) => Json(canvas).into_response(),
        Ok(None) => not_found(),
        Err(err) => server_error(err),
    }
}

async fn public_canvas(State(state): State<AppState>, Path(id): Path<Uuid>) -> Response {
    let Some(db) = &state.db else {
        return db_unavailable();
    };
    match service::get(db, id).await {
        Ok(Some(canvas)) => Html(canvas.content_html).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, Html("<p>Not found.</p>")).into_response(),
        Err(err) => server_error(err),
    }
}

fn bad_request(message: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(json!({ "error": message }))).into_response()
}

fn not_found() -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": "canvas not found" })),
    )
        .into_response()
}

fn server_error(err: sea_orm::DbErr) -> Response {
    tracing::error!("canvas db error: {err}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": "database error" })),
    )
        .into_response()
}

fn db_unavailable() -> Response {
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(json!({ "error": "database is not connected" })),
    )
        .into_response()
}
