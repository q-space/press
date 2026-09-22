//! Job producer -- enqueues newsletter-send jobs. Pushing the accepted
//! job to Redis for the worker to pick up is still not built (original
//! stub's scope, and no Redis connection is wired into `main.rs` yet --
//! same reason `db.rs::connect` isn't called either).
//!
//! What IS built here (BB26091206): the decision this producer must make
//! before a job can go anywhere -- is this send within the publication's
//! delivery allowance right now, metered on subscribers delivered to, not
//! posts written (canon D-008). This is the "tier enforcement in the
//! email queue" build item.
//!
//! Never silently drops subscribers or emails. An over-limit post is
//! still recorded, as a job with status `held_over_limit` -- its history
//! exists, nothing about it is destroyed, and it becomes sendable again
//! automatically once `worker::try_release_held_job` confirms the
//! publication is back within allowance. See `worker.rs` for that side.

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, Statement};
use uuid::Uuid;

use crate::delivery::metering::{meter_send, DeliveryDecision};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnqueueOutcome {
    /// Job created with status `pending` -- the worker is free to pick it
    /// up and actually send.
    Queued { job_id: Uuid },
    /// Job created with status `held_over_limit`. The send does NOT go
    /// out. This function guarantees the hold is recorded on the job row
    /// (`held_at`/`held_reason`) -- it does NOT itself notify the
    /// creator; that belongs to whatever calls this (posts::service, once
    /// Feature 1's publish flow exists) so the notification can go out on
    /// whatever channel that service already uses (email, in-app, both).
    /// `held_reason` is the exact message to surface there.
    Held { job_id: Uuid, held_reason: String },
}

/// Enqueues a post's newsletter send. Idempotent-by-constraint, not by
/// this function: `post_id` is UNIQUE on `newsletter_send_jobs`
/// (migrations/005_delivery_metering.sql), so a second call for the same
/// post returns a DB error rather than a second job -- re-publish/resend
/// flows deciding what to do with that error is Feature 1 scope, not
/// this row's.
pub async fn enqueue_newsletter_send(
    db: &DatabaseConnection,
    post_id: Uuid,
    publication_id: Uuid,
) -> Result<EnqueueOutcome, DbErr> {
    let decision = meter_send(db, publication_id).await?;

    let (status, held_reason) = match decision {
        DeliveryDecision::WithinAllowance { .. } => ("pending", None),
        DeliveryDecision::OverLimit {
            tier,
            allowance,
            billable_subscriber_count,
        } => {
            let reason = format!(
                "{} plan allows {} subscribers; this publication currently has {}. \
                 Send held -- upgrade your delivery plan, or reduce your confirmed \
                 subscriber count, to release it.",
                tier.as_db_str(),
                allowance,
                billable_subscriber_count
            );
            ("held_over_limit", Some(reason))
        }
    };

    let (tier, allowance, billable_subscriber_count) = match decision {
        DeliveryDecision::WithinAllowance {
            tier,
            allowance,
            billable_subscriber_count,
        }
        | DeliveryDecision::OverLimit {
            tier,
            allowance,
            billable_subscriber_count,
        } => (tier, allowance, billable_subscriber_count),
    };

    let job_id = Uuid::now_v7();
    db.execute(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
        INSERT INTO newsletter_send_jobs
            (id, post_id, publication_id, status, billable_subscriber_count,
             delivery_plan, delivery_allowance, held_at, held_reason)
        VALUES ($1, $2, $3, $4, $5, $6, $7,
                CASE WHEN $4 = 'held_over_limit' THEN NOW() ELSE NULL END,
                $8)
        "#,
        [
            job_id.into(),
            post_id.into(),
            publication_id.into(),
            status.into(),
            (billable_subscriber_count as i64).into(),
            tier.as_db_str().into(),
            (allowance as i64).into(),
            held_reason.clone().into(),
        ],
    ))
    .await?;

    Ok(match held_reason {
        Some(reason) => EnqueueOutcome::Held {
            job_id,
            held_reason: reason,
        },
        None => EnqueueOutcome::Queued { job_id },
    })
}
