//! Per-publication subscriber counting and tier-allowance decisions, at
//! send time. Canon: D-008 -- "metered on subscribers delivered to, not
//! posts written."
//!
//! No SeaORM entities exist yet for `publications`/`subscribers` (see
//! db.rs's own doc comment: entity generation is Week 2 work, once this
//! project has a wired DB connection at all). This queries with raw SQL
//! through `ConnectionTrait`, the same escape hatch SeaORM documents for
//! call sites that predate entity generation. Once entities land, the two
//! query functions below are the only things that need rewriting -- the
//! decision logic (`decide`) is entity-agnostic and DB-agnostic, and is
//! exercised directly by this file's unit tests without a database.

use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, FromQueryResult, Statement};
use uuid::Uuid;

use super::tiers::{resolve_allowance, DeliveryTier};

#[derive(Debug, FromQueryResult)]
struct PublicationPlanRow {
    delivery_plan: String,
    institutional_allowance_override: Option<i32>,
}

#[derive(Debug, FromQueryResult)]
struct CountRow {
    count: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryDecision {
    /// Send proceeds. Carries the numbers so the caller (queue.rs) can
    /// persist them on the send job without a second query.
    WithinAllowance {
        tier: DeliveryTier,
        allowance: u32,
        billable_subscriber_count: u32,
    },
    /// Send must NOT go out as a normal send. The queue holds the job
    /// (queue.rs) rather than dropping any subscriber or email -- see
    /// this row's task description: "warn and hold ... never silently
    /// drop".
    OverLimit {
        tier: DeliveryTier,
        allowance: u32,
        billable_subscriber_count: u32,
    },
}

/// Pure decision: given a count and an allowance, is this send within
/// bounds. Split out from `meter_send` specifically so it can be unit
/// tested without a database -- see tests below.
pub fn decide(tier: DeliveryTier, allowance: u32, billable_subscriber_count: u32) -> DeliveryDecision {
    if billable_subscriber_count <= allowance {
        DeliveryDecision::WithinAllowance {
            tier,
            allowance,
            billable_subscriber_count,
        }
    } else {
        DeliveryDecision::OverLimit {
            tier,
            allowance,
            billable_subscriber_count,
        }
    }
}

/// Counts subscribers a send would actually reach: confirmed, and not
/// unsubscribed. Deliberately NOT "every row in `subscribers`" -- an
/// unconfirmed or unsubscribed row is not someone a send delivers to, and
/// counting them would meter a publication for people it never reaches,
/// which is exactly the "posts written" mismeasurement D-008 rejects in
/// the other direction.
pub async fn count_billable_subscribers(
    db: &DatabaseConnection,
    publication_id: Uuid,
) -> Result<u32, DbErr> {
    let row = CountRow::find_by_statement(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
        SELECT COUNT(*) AS count
        FROM subscribers
        WHERE publication_id = $1
          AND confirmed = TRUE
          AND tier <> 'unsubscribed'
        "#,
        [publication_id.into()],
    ))
    .one(db)
    .await?
    .expect("COUNT(*) always returns exactly one row");

    Ok(row.count.max(0) as u32)
}

/// Resolves the publication's current plan and numeric allowance
/// (Institutional override folded in).
pub async fn resolve_publication_allowance(
    db: &DatabaseConnection,
    publication_id: Uuid,
) -> Result<(DeliveryTier, u32), DbErr> {
    let row = PublicationPlanRow::find_by_statement(Statement::from_sql_and_values(
        db.get_database_backend(),
        r#"
        SELECT delivery_plan, institutional_allowance_override
        FROM publications
        WHERE id = $1
        "#,
        [publication_id.into()],
    ))
    .one(db)
    .await?
    .ok_or_else(|| DbErr::RecordNotFound(format!("publication {publication_id} not found")))?;

    // The CHECK constraint on delivery_plan means this only trips on a
    // value this code doesn't know about yet (a new tier added to the DB
    // but not to DeliveryTier). Fail closed to Free -- the most
    // conservative allowance -- rather than to unmetered, matching the
    // "warn and hold, never silently drop" spirit of the over-limit path.
    let tier = DeliveryTier::from_db_str(&row.delivery_plan).unwrap_or(DeliveryTier::Free);
    let allowance = resolve_allowance(
        tier,
        row.institutional_allowance_override.map(|v| v.max(0) as u32),
    );

    Ok((tier, allowance))
}

/// The one call site queue.rs needs: count subscribers, resolve the plan,
/// decide. Two queries rather than a JOIN is deliberate -- the count runs
/// against `subscribers`, the plan against `publications`, and keeping
/// them separate keeps each independently reusable (the in-product usage
/// surface in `handlers.rs` calls both the same way, with no send job in
/// flight).
pub async fn meter_send(
    db: &DatabaseConnection,
    publication_id: Uuid,
) -> Result<DeliveryDecision, DbErr> {
    let billable_subscriber_count = count_billable_subscribers(db, publication_id).await?;
    let (tier, allowance) = resolve_publication_allowance(db, publication_id).await?;
    Ok(decide(tier, allowance, billable_subscriber_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_exactly_the_allowance_is_within_bounds() {
        // 1,000 subscribers on Free is the canon's own boundary example
        // ("a creator publishing monthly to 20,000 pays") -- exactly at
        // the line must not be treated as over it.
        let decision = decide(DeliveryTier::Free, 1_000, 1_000);
        assert!(matches!(decision, DeliveryDecision::WithinAllowance { .. }));
    }

    #[test]
    fn one_over_the_allowance_is_over_limit() {
        let decision = decide(DeliveryTier::Free, 1_000, 1_001);
        assert!(matches!(decision, DeliveryDecision::OverLimit { .. }));
    }

    #[test]
    fn decision_carries_the_numbers_through() {
        let decision = decide(DeliveryTier::Growth, 10_000, 12_345);
        match decision {
            DeliveryDecision::OverLimit {
                tier,
                allowance,
                billable_subscriber_count,
            } => {
                assert_eq!(tier, DeliveryTier::Growth);
                assert_eq!(allowance, 10_000);
                assert_eq!(billable_subscriber_count, 12_345);
            }
            other => panic!("expected OverLimit, got {other:?}"),
        }
    }

    #[test]
    fn zero_subscribers_is_always_within_bounds() {
        let decision = decide(DeliveryTier::Institutional, 0, 0);
        assert!(matches!(decision, DeliveryDecision::WithinAllowance { .. }));
    }
}
