//! In-product surfacing of where a creator sits against their delivery
//! allowance (this row's fourth build item: "Clear in-product surfacing
//! of where a creator sits against their allowance").
//!
//! Not yet mounted on `main.rs`'s router -- same reason every other
//! handler module in this repo isn't (db.rs's own doc comment): there is
//! no `DatabaseConnection` in the boot sequence yet, so there is nothing
//! for `axum::extract::State` to extract. Ready to mount the moment that
//! lands:
//!
//! ```ignore
//! .route("/publications/:id/delivery-status", get(delivery::handlers::delivery_status))
//! .with_state(db_connection)
//! ```

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::DatabaseConnection;
use serde::Serialize;
use uuid::Uuid;

use super::metering::{count_billable_subscribers, resolve_publication_allowance};
use super::tiers::DeliveryTier;

#[derive(Debug, Serialize)]
pub struct DeliveryStatus {
    pub plan: DeliveryTier,
    pub allowance: u32,
    pub billable_subscriber_count: u32,
    pub percent_used: f32,
    /// True once `billable_subscriber_count > allowance` -- i.e. the next
    /// send would be (or an in-flight one already is) held rather than
    /// sent. Surfaced explicitly rather than left for the client to infer
    /// from the other two numbers, since "am I currently blocked" is the
    /// one fact a creator needs at a glance.
    pub over_limit: bool,
    pub next_tier: Option<DeliveryTier>,
}

/// `GET /publications/:id/delivery-status`. Returns 404 if the
/// publication doesn't exist, 500 on a real DB error -- never a
/// silently-wrong number, matching the "never silently drop" spirit of
/// the rest of this row for the read path too.
pub async fn delivery_status(
    State(db): State<DatabaseConnection>,
    Path(publication_id): Path<Uuid>,
) -> impl IntoResponse {
    let count = match count_billable_subscribers(&db, publication_id).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("delivery_status: subscriber count failed: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to load delivery status",
            )
                .into_response();
        }
    };

    let (plan, allowance) = match resolve_publication_allowance(&db, publication_id).await {
        Ok(v) => v,
        Err(sea_orm::DbErr::RecordNotFound(_)) => {
            return (StatusCode::NOT_FOUND, "publication not found").into_response();
        }
        Err(e) => {
            tracing::error!("delivery_status: plan lookup failed: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to load delivery status",
            )
                .into_response();
        }
    };

    let percent_used = if allowance == 0 {
        0.0
    } else {
        (count as f32 / allowance as f32) * 100.0
    };

    let status = DeliveryStatus {
        plan,
        allowance,
        billable_subscriber_count: count,
        percent_used,
        over_limit: count > allowance,
        next_tier: plan.next_tier(),
    };

    Json(status).into_response()
}
