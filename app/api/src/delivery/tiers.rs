//! Delivery tier definitions. Pure, DB-free, unit-testable on their own --
//! see the tests below, which are the only tests in this row that run
//! without a Postgres connection.
//!
//! Canon pricing table (work/_arc/press/canon-canvas/
//! 20260915_project_development_canonical_qspace_press_v2_1_0.md,
//! "Pricing Strategy"):
//!
//! | Tier          | Price            | Delivery allowance     |
//! |---------------|------------------|-------------------------|
//! | Free          | KES 0/month      | Up to 1,000 subscribers |
//! | Growth        | KES 900/month    | Up to 10,000 subscribers|
//! | Scale         | KES 3,500/month  | Up to 50,000 subscribers|
//! | Institutional | From KES 25,000  | Negotiated              |
//!
//! Revenue share (12% / 10% / 7% tiered take rate on monetization) is a
//! SEPARATE billing dimension -- it activates on monetization regardless
//! of delivery plan, and a creator on Free who monetises pays only the
//! take rate; a creator on Scale who never monetises pays only the flat
//! fee. This module must never grow a dependency on take-rate logic.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryTier {
    Free,
    Growth,
    Scale,
    Institutional,
}

impl DeliveryTier {
    /// Fixed subscriber allowance for the three metered tiers.
    /// Institutional is "Negotiated" per the canon pricing table -- it has
    /// no constant here; `resolve_allowance` below folds in the
    /// per-publication override instead.
    pub fn fixed_allowance(self) -> Option<u32> {
        match self {
            DeliveryTier::Free => Some(1_000),
            DeliveryTier::Growth => Some(10_000),
            DeliveryTier::Scale => Some(50_000),
            DeliveryTier::Institutional => None,
        }
    }

    /// `None` for Institutional ("From KES 25,000/month", negotiated, not
    /// a fixed number).
    pub fn monthly_price_kes(self) -> Option<u32> {
        match self {
            DeliveryTier::Free => Some(0),
            DeliveryTier::Growth => Some(900),
            DeliveryTier::Scale => Some(3_500),
            DeliveryTier::Institutional => None,
        }
    }

    pub fn as_db_str(self) -> &'static str {
        match self {
            DeliveryTier::Free => "free",
            DeliveryTier::Growth => "growth",
            DeliveryTier::Scale => "scale",
            DeliveryTier::Institutional => "institutional",
        }
    }

    /// Matches the CHECK constraint on `publications.delivery_plan`
    /// (migrations/004_delivery_metering.sql). Returns `None` for any
    /// value the DB wouldn't accept either -- callers should fail closed
    /// (see `metering::resolve_publication_allowance`'s doc comment for
    /// why "closed" here means the most conservative tier, Free).
    pub fn from_db_str(s: &str) -> Option<Self> {
        match s {
            "free" => Some(DeliveryTier::Free),
            "growth" => Some(DeliveryTier::Growth),
            "scale" => Some(DeliveryTier::Scale),
            "institutional" => Some(DeliveryTier::Institutional),
            _ => None,
        }
    }

    /// The next tier up, for "upgrade to X" messaging in both the
    /// over-limit hold reason and the in-product usage surface. `None`
    /// once already on Institutional -- there is nowhere further to
    /// suggest.
    pub fn next_tier(self) -> Option<Self> {
        match self {
            DeliveryTier::Free => Some(DeliveryTier::Growth),
            DeliveryTier::Growth => Some(DeliveryTier::Scale),
            DeliveryTier::Scale => Some(DeliveryTier::Institutional),
            DeliveryTier::Institutional => None,
        }
    }
}

/// Resolves a publication's actual numeric allowance, folding in the
/// Institutional override (`publications.institutional_allowance_override`,
/// ignored for every other tier). An Institutional publication with no
/// override recorded yet (negotiation not finished) resolves to 0 --
/// deliberately the most conservative number, not an assumed default,
/// since guessing a number for a negotiated contract is exactly the kind
/// of silent behaviour this row's over-limit path is built to avoid.
pub fn resolve_allowance(tier: DeliveryTier, institutional_override: Option<u32>) -> u32 {
    match tier {
        DeliveryTier::Institutional => institutional_override.unwrap_or(0),
        other => other
            .fixed_allowance()
            .expect("every non-Institutional tier has a fixed allowance"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_allowances_match_canon_table() {
        assert_eq!(DeliveryTier::Free.fixed_allowance(), Some(1_000));
        assert_eq!(DeliveryTier::Growth.fixed_allowance(), Some(10_000));
        assert_eq!(DeliveryTier::Scale.fixed_allowance(), Some(50_000));
        assert_eq!(DeliveryTier::Institutional.fixed_allowance(), None);
    }

    #[test]
    fn prices_match_canon_table() {
        assert_eq!(DeliveryTier::Free.monthly_price_kes(), Some(0));
        assert_eq!(DeliveryTier::Growth.monthly_price_kes(), Some(900));
        assert_eq!(DeliveryTier::Scale.monthly_price_kes(), Some(3_500));
        assert_eq!(DeliveryTier::Institutional.monthly_price_kes(), None);
    }

    #[test]
    fn db_str_roundtrips() {
        for tier in [
            DeliveryTier::Free,
            DeliveryTier::Growth,
            DeliveryTier::Scale,
            DeliveryTier::Institutional,
        ] {
            assert_eq!(DeliveryTier::from_db_str(tier.as_db_str()), Some(tier));
        }
    }

    #[test]
    fn unknown_db_str_resolves_to_none() {
        assert_eq!(DeliveryTier::from_db_str("bogus"), None);
    }

    #[test]
    fn next_tier_chain_ends_at_institutional() {
        assert_eq!(DeliveryTier::Free.next_tier(), Some(DeliveryTier::Growth));
        assert_eq!(DeliveryTier::Growth.next_tier(), Some(DeliveryTier::Scale));
        assert_eq!(
            DeliveryTier::Scale.next_tier(),
            Some(DeliveryTier::Institutional)
        );
        assert_eq!(DeliveryTier::Institutional.next_tier(), None);
    }

    #[test]
    fn resolve_allowance_uses_fixed_number_for_metered_tiers() {
        assert_eq!(resolve_allowance(DeliveryTier::Free, None), 1_000);
        assert_eq!(resolve_allowance(DeliveryTier::Growth, Some(999_999)), 10_000);
    }

    #[test]
    fn resolve_allowance_institutional_uses_override_or_zero() {
        assert_eq!(resolve_allowance(DeliveryTier::Institutional, Some(75_000)), 75_000);
        assert_eq!(resolve_allowance(DeliveryTier::Institutional, None), 0);
    }
}
