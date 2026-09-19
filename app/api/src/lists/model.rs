//! Wire types for structured list documents. Plain structs, not SeaORM
//! entities yet -- entities get generated from `migrations/004_structured_lists.sql`
//! once `db.rs` is wired in (see `mod.rs`'s "Not yet wired" note).

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredList {
    pub id: Uuid,
    pub title: String,
    pub owner_id: Uuid,
    pub items: Vec<StructuredListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredListItem {
    pub id: Uuid,
    pub list_id: Uuid,
    pub position: i32,
    pub text: String,
    pub bucket: Option<String>,
    pub status: String,
    pub owner_name: Option<String>,
    pub note: Option<String>,
    /// Optimistic-lock counter. A patch must be read against this exact
    /// value -- see `service::apply_patch`.
    pub version: i32,
}

/// Body of a PATCH to one item. `version` is the value the client last
/// read; every other field is optional so a caller can update just the
/// fields that changed.
#[derive(Debug, Clone, Deserialize)]
pub struct ItemPatch {
    pub version: i32,
    pub text: Option<String>,
    pub bucket: Option<String>,
    pub status: Option<String>,
    pub owner_name: Option<String>,
    pub note: Option<String>,
}

/// A share link's editing capability -- `can_edit = false` is the default,
/// matching Canvas's current read-only-by-default link. See `mod.rs` §3.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLink {
    pub id: Uuid,
    pub list_id: Uuid,
    pub token: String,
    pub can_edit: bool,
}
