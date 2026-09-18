// app/api/src/push/handlers.rs
//
// CHANGELOG:
// - BB26091207: initial subscribe/unsubscribe handlers.
//! POST /push/subscribe, POST /push/unsubscribe (BB26091207). Reached only
//! via the Next.js BFF routes under `app/web/app/api/push/`, never
//! directly from the browser (canon D-010 BFF pattern, same as every
//! other `app/api/src/*/handlers.rs`).

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use std::sync::Arc;

use super::store::{PushStore, PushSubscriptionRecord};

#[derive(Deserialize)]
pub struct SubscribeKeys {
    pub p256dh: String,
    pub auth: String,
}

#[derive(Deserialize)]
pub struct SubscribePayload {
    pub endpoint: String,
    pub keys: SubscribeKeys,
}

#[derive(Deserialize)]
pub struct SubscribeRequest {
    pub handle: String,
    pub subscription: SubscribePayload,
}

pub async fn subscribe(
    State(store): State<Arc<PushStore>>,
    Json(req): Json<SubscribeRequest>,
) -> Result<StatusCode, StatusCode> {
    if req.handle.trim().is_empty()
        || req.subscription.endpoint.trim().is_empty()
        || req.subscription.keys.p256dh.trim().is_empty()
        || req.subscription.keys.auth.trim().is_empty()
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    store.upsert(PushSubscriptionRecord {
        handle: req.handle,
        endpoint: req.subscription.endpoint,
        p256dh: req.subscription.keys.p256dh,
        auth: req.subscription.keys.auth,
        subscribed_at: chrono::Utc::now(),
    });

    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
pub struct UnsubscribeRequest {
    #[allow(dead_code)] // kept for future per-handle auditing; not needed to remove by endpoint
    pub handle: Option<String>,
    pub endpoint: String,
}

/// Idempotent by design -- unsubscribing an endpoint that was never (or no
/// longer) stored still returns 200, since the caller's goal ("this
/// browser should stop being pushed to") is already true either way.
pub async fn unsubscribe(
    State(store): State<Arc<PushStore>>,
    Json(req): Json<UnsubscribeRequest>,
) -> StatusCode {
    if req.endpoint.trim().is_empty() {
        return StatusCode::BAD_REQUEST;
    }
    store.remove(&req.endpoint);
    StatusCode::OK
}
