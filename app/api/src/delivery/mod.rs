//! Delivery metering (BB26091206) -- meters newsletter delivery on
//! subscribers delivered to, not posts written. Canon: work/_arc/press/
//! canon-canvas/20260915_project_development_canonical_qspace_press_v2_1_0.md,
//! "Pricing Strategy" section + Decision Log D-008.
//!
//! Deliberately its own top-level module, not nested under `email` or
//! `audience`: it is consumed by `email::queue` (enforcement) and will
//! eventually be consumed by Feature 2 / `audience` (subscriber-facing
//! surfacing) and by billing/Feature 2 once revenue share exists -- see
//! this row's own findings on Feature 2 not being built yet. Keeping tier
//! logic here, with no dependency in either direction, is what makes this
//! independently mergeable ahead of Feature 2 rather than bolted onto it.
//!
//! `tiers` has zero knowledge of revenue share / take rate and must stay
//! that way -- delivery tier and revenue share are independent billing
//! dimensions per the canon; nobody pays twice for the same thing.

pub mod handlers;
pub mod metering;
pub mod tiers;
