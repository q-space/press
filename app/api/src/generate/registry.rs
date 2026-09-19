//! The archetype registry -- namespaced per engagement/project per canon
//! §1/§3, `_common` for archetypes reusable across engagements. Ported
//! from `scope/lib/generate/registry.js`, adapted for Rust: that file
//! scans `archetypes/` on disk at require-time; Rust has no equivalent of
//! dynamically loading a directory of modules, so this instead lists each
//! archetype explicitly. "What archetypes exist" is therefore answered by
//! what's registered here rather than what's on disk -- same effective
//! guarantee (compiling this file IS the scan), just resolved at compile
//! time instead of first-request time.
//!
//! Only `_common` is populated today. Per-engagement namespaces don't
//! exist yet on the Qpress side -- BB26091205 (the thin-client API) is
//! what will resolve a real namespace against Qpress's own data model;
//! until then, every lookup effectively falls through to `_common`, which
//! mirrors registry.js's own fallback behavior for a namespace with no
//! archetypes of its own.

use super::archetype::Archetype;
use super::archetypes::decision_brief::DecisionBrief;
use super::archetypes::email::Email;
use super::archetypes::formal_letter::FormalLetter;
use super::archetypes::invoice::Invoice;
use super::archetypes::meeting_notes::MeetingNotes;
use super::archetypes::multipage_report::MultipageReport;
use super::archetypes::proposal::Proposal;
use super::archetypes::seed_data_catalogue::SeedDataCatalogue;
use super::archetypes::single_page_memo::SinglePageMemo;
use super::archetypes::weekly_status_brief::WeeklyStatusBrief;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;

fn common_archetypes() -> Vec<Arc<dyn Archetype>> {
    vec![
        Arc::new(SinglePageMemo),
        Arc::new(DecisionBrief),
        Arc::new(Email),
        Arc::new(FormalLetter),
        Arc::new(Invoice),
        Arc::new(MeetingNotes),
        Arc::new(MultipageReport),
        Arc::new(Proposal),
        Arc::new(SeedDataCatalogue),
        Arc::new(WeeklyStatusBrief),
        // Remaining 1: page-truth-brief -- not yet located/ported (client
        // content, not infrastructure -- see BB26091203's own note).
    ]
}

struct Registry {
    common: HashMap<&'static str, Arc<dyn Archetype>>,
}

fn registry() -> &'static Registry {
    static REGISTRY: OnceLock<Registry> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let mut common = HashMap::new();
        for a in common_archetypes() {
            common.insert(a.id(), a);
        }
        Registry { common }
    })
}

#[derive(Debug)]
pub struct RegistryError(pub String);

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for RegistryError {}

/// `namespace` is accepted (not yet ignored outright) so call sites don't
/// need to change again once real per-engagement namespaces exist -- today
/// every lookup resolves against `_common` only, since no other namespace
/// is registered yet.
pub fn get_archetype(_namespace: &str, archetype_id: &str) -> Result<Arc<dyn Archetype>, RegistryError> {
    registry()
        .common
        .get(archetype_id)
        .cloned()
        .ok_or_else(|| RegistryError(format!("no archetype \"{archetype_id}\" in _common")))
}

pub fn list_archetypes(_namespace: &str) -> Vec<Arc<dyn Archetype>> {
    registry().common.values().cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_archetype_finds_a_common_archetype() {
        let a = get_archetype("_common", "single-page-memo").unwrap();
        assert_eq!(a.id(), "single-page-memo");
    }

    #[test]
    fn get_archetype_errors_on_an_unknown_id() {
        let err = get_archetype("_common", "no-such-thing").unwrap_err();
        assert!(err.0.contains("no-such-thing"));
    }
}
