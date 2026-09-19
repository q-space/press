//! Job consumer -- sends queued newsletter emails via Resend. The actual
//! Redis consumption loop is still not built (original stub's scope --
//! no Redis connection is wired into `main.rs` yet, same reason `db.rs`
//! isn't: see that file's doc comment).
//!
//! What IS built here (BB26091206): the re-check a held job needs before
//! it can ever become a normal send. A job enqueued as `held_over_limit`
//! (queue.rs) must never be quietly flipped to `pending` by a timer or a
//! retry without re-checking reality -- it is released only by
//! re-metering the publication and confirming it is now within allowance
//! (creator upgraded their plan, or their confirmed subscriber count
//! fell). This is the "queue paused" half of the over-limit path; the
//! "creator notified" half belongs to whatever calls `enqueue_newsletter_send`
//! (queue.rs), since that is where the reason string is already produced.

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, FromQueryResult, Statement};
use uuid::Uuid;

use crate::delivery::metering::{meter_send, DeliveryDecision};
use crate::delivery::tiers::DeliveryTier;

#[derive(Debug, FromQueryResult)]
struct HeldJobRow {
    id: Uuid,
    publication_id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseOutcome {
    /// Still over the (possibly changed) allowance -- job stays held,
    /// nothing is sent, no email or subscriber is dropped.
    StillHeld,
    /// Re-metering confirms the publication is now within allowance --
    /// job flips to `pending` and can be picked up by the send loop.
    Released { tier: DeliveryTier, allowance: u32 },
}

/// Re-evaluates one held job against the publication's CURRENT plan and
/// subscriber count -- never the numbers captured at original enqueue
/// time, which are historical record on the job row, not live truth --
/// and releases it if it now fits.
pub async fn try_release_held_job(
    db: &DatabaseConnection,
    job_id: Uuid,
    publication_id: Uuid,
) -> Result<ReleaseOutcome, DbErr> {
    match meter_send(db, publication_id).await? {
        DeliveryDecision::OverLimit { .. } => Ok(ReleaseOutcome::StillHeld),
        DeliveryDecision::WithinAllowance { tier, allowance, .. } => {
            db.execute(Statement::from_sql_and_values(
                db.get_database_backend(),
                r#"
                UPDATE newsletter_send_jobs
                SET status = 'pending', held_at = NULL, updated_at = NOW()
                WHERE id = $1 AND status = 'held_over_limit'
                "#,
                [job_id.into()],
            ))
            .await?;
            Ok(ReleaseOutcome::Released { tier, allowance })
        }
    }
}

/// Lists every currently-held job, oldest first, for whatever schedules
/// the release check (a periodic sweep, or a hook off plan-upgrade /
/// subscriber-count events -- not decided by this row; both are valid
/// callers of `try_release_held_job` above, and neither is built yet).
pub async fn list_held_jobs(db: &DatabaseConnection) -> Result<Vec<(Uuid, Uuid)>, DbErr> {
    let rows = HeldJobRow::find_by_statement(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
        SELECT id, publication_id
        FROM newsletter_send_jobs
        WHERE status = 'held_over_limit'
        ORDER BY held_at ASC
        "#,
        [],
    ))
    .all(db)
    .await?;

    Ok(rows.into_iter().map(|r| (r.id, r.publication_id)).collect())
}
