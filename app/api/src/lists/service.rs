//! Optimistic-locking business logic for structured list items. Pure
//! functions, no DB access -- the DB-backed compare-and-swap (an `UPDATE
//! ... WHERE id = $1 AND version = $2`) is the same shape as
//! `apply_patch` below and gets wired in once `db.rs` connects (see
//! `mod.rs`'s "Not yet wired" note). Kept pure and tested here so the
//! conflict semantics are pinned down before there is a database to hide
//! a bug behind.

use super::model::{ItemPatch, StructuredListItem};

pub enum PatchOutcome {
    Applied(StructuredListItem),
    /// The patch's `version` no longer matches the stored item -- reject
    /// with the current row so the caller can reload and retry, per
    /// `mod.rs` §2. Never silently drop one editor's change.
    Conflict(StructuredListItem),
}

/// Apply `patch` to `item` if and only if `patch.version` matches
/// `item.version`. On success, bumps `version` by exactly 1 so a second
/// patch built against the OLD version cannot also apply.
pub fn apply_patch(item: &StructuredListItem, patch: &ItemPatch) -> PatchOutcome {
    if patch.version != item.version {
        return PatchOutcome::Conflict(item.clone());
    }

    let mut updated = item.clone();
    if let Some(text) = &patch.text {
        updated.text = text.clone();
    }
    if patch.bucket.is_some() {
        updated.bucket = patch.bucket.clone();
    }
    if let Some(status) = &patch.status {
        updated.status = status.clone();
    }
    if patch.owner_name.is_some() {
        updated.owner_name = patch.owner_name.clone();
    }
    if patch.note.is_some() {
        updated.note = patch.note.clone();
    }
    updated.version += 1;

    PatchOutcome::Applied(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn item(version: i32) -> StructuredListItem {
        StructuredListItem {
            id: Uuid::new_v4(),
            list_id: Uuid::new_v4(),
            position: 0,
            text: "original".into(),
            bucket: Some("TEXT".into()),
            status: "open".into(),
            owner_name: None,
            note: None,
            version,
        }
    }

    #[test]
    fn matching_version_applies_and_bumps() {
        let current = item(1);
        let patch = ItemPatch {
            version: 1,
            text: Some("edited".into()),
            bucket: None,
            status: None,
            owner_name: None,
            note: None,
        };
        match apply_patch(&current, &patch) {
            PatchOutcome::Applied(updated) => {
                assert_eq!(updated.text, "edited");
                assert_eq!(updated.version, 2);
            }
            PatchOutcome::Conflict(_) => panic!("expected apply, got conflict"),
        }
    }

    #[test]
    fn stale_version_conflicts_without_mutating() {
        let current = item(3);
        let patch = ItemPatch {
            version: 2, // stale -- someone else already wrote version 3
            text: Some("stale edit".into()),
            bucket: None,
            status: None,
            owner_name: None,
            note: None,
        };
        match apply_patch(&current, &patch) {
            PatchOutcome::Conflict(returned) => {
                assert_eq!(returned.version, 3);
                assert_eq!(returned.text, "original");
            }
            PatchOutcome::Applied(_) => panic!("expected conflict, got apply"),
        }
    }

    #[test]
    fn two_sequential_patches_from_the_same_read_only_one_applies() {
        let current = item(1);
        let patch = ItemPatch {
            version: 1,
            text: Some("first writer".into()),
            bucket: None,
            status: None,
            owner_name: None,
            note: None,
        };
        let after_first = match apply_patch(&current, &patch) {
            PatchOutcome::Applied(u) => u,
            PatchOutcome::Conflict(_) => panic!("first patch should apply"),
        };

        // Second editor read the SAME original (version 1) and patches
        // against it too -- must conflict now that version is 2.
        let second_patch = ItemPatch {
            version: 1,
            text: Some("second writer".into()),
            bucket: None,
            status: None,
            owner_name: None,
            note: None,
        };
        match apply_patch(&after_first, &second_patch) {
            PatchOutcome::Conflict(returned) => assert_eq!(returned.text, "first writer"),
            PatchOutcome::Applied(_) => panic!("second patch should conflict"),
        }
    }
}
