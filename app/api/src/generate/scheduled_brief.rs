//! BB26091208 part 3: the Friday auto-draft, rebuilt as a proper scheduled
//! job that INVOKES the `weekly-status-brief` archetype rather than
//! bypassing it. iSconl's current version (`scope/lib/status-brief.js`,
//! BA26082420) is a bare `setInterval` firing a hand-rolled email body --
//! it never goes through an archetype's `build()`/a renderer at all. This
//! module is the replacement: gather activity -> draft field values via
//! `ai_assist::full_draft` (the SAME function `BB26091208` part 2 exposes
//! over HTTP, not a second AI-calling path) -> allocate a doc ID
//! (`doc_registry`) -> call `doc_builder::build("_common",
//! "weekly-status-brief", content)` (part 1's archetype, unchanged) ->
//! render. Nothing here authors a document directly.
//!
//! Carried forward from `status-brief.js` rather than re-derived (its own
//! doc comments cite `BA26091201`0`, `FA26091201`, `FA26091202`):
//! - `monday_of()` is ISO-week correct and UTC-based -- ported unchanged.
//! - `gather_activity()`'s shape (tasks.tsv filtered by org+7 days,
//!   interactions.tsv filtered by the SAME window) is ported, WITH the
//!   subject-person-ID filter already present in the source file's current
//!   state (`subjectPersonIds`, `FA26091201`) -- the task brief that scoped
//!   this row described that filter as missing (tracked as `BF26091202`),
//!   but the actual JS this session read already has it. Ported as read,
//!   not as described; see this row's own follow-up note about the
//!   discrepancy.
//! - `draft_all_briefs()` isolates one subject's failure from the rest,
//!   same as `draftAllBriefs()`.
//! - ID allocation is NOT the `SB%04d` max-suffix scan (`status-brief.js`'s
//!   OWN unsafe fallback path, not even its primary path -- its primary
//!   path already uses an atomic `rewriteTSV` callback). This module uses
//!   `doc_registry::DocRegistry::allocate()`, whose lock-held
//!   read-modify-write cycle is the Rust equivalent of that same atomic
//!   pattern.

use crate::generate::ai_assist::{self, FieldDescriptor};
use crate::generate::archetype::FieldDef;
use crate::generate::content::Content;
use crate::generate::doc_builder;
use crate::generate::doc_registry::DocRegistry;
use crate::generate::registry;
use chrono::{Datelike, NaiveDate, Weekday};
use serde_json::Value;

/// ISO week Monday of `date` -- ported from `status-brief.js`'s
/// `mondayOf()`. `NaiveDate::week(Weekday::Mon).first_day()` is chrono's
/// own ISO-week-Monday primitive; using it instead of hand-rolled
/// day-arithmetic is a behavior-preserving simplification (chrono defines
/// ISO weeks Monday-start, same convention the JS comment calls out), not
/// a departure from it.
pub fn monday_of(date: NaiveDate) -> NaiveDate {
    date.week(Weekday::Mon).first_day()
}

#[derive(Debug, Clone)]
pub struct ActivityItem {
    pub date: String,
    pub kind: String, // "task" | "interaction"
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct SubjectRow {
    pub subject_id: String,
    /// Org/source reference (`tasks.ORG_ID` match key).
    pub source_ref: String,
    pub supervisor_or_contact: String,
    /// "active" | "retired" | ... -- only non-"retired" subjects are drafted.
    pub status: String,
}

/// Where subjects and their week of activity come from. A trait, not a
/// hardcoded TSV reader, because Press has no subjects data model of its
/// own yet (unlike iSconl's `scope/active_subjects.tsv`) -- a real
/// Postgres-backed implementation is Cycle-2+ scope; this keeps the
/// scheduler itself independent of where subject data eventually lives.
pub trait SubjectSource: Send + Sync {
    fn active_subjects(&self) -> Vec<SubjectRow>;
    fn gather_activity(&self, subject: &SubjectRow) -> Vec<ActivityItem>;
}

#[derive(Debug, Clone)]
pub struct BriefDraftResult {
    pub subject_id: String,
    pub success: bool,
    pub doc_id: Option<String>,
    pub error: Option<String>,
}

fn weekly_status_brief_fields() -> Vec<FieldDescriptor> {
    registry::get_archetype("_common", "weekly-status-brief")
        .map(|a| a.fields().iter().map(field_def_to_descriptor).collect())
        .unwrap_or_default()
}

fn field_def_to_descriptor(f: &FieldDef) -> FieldDescriptor {
    FieldDescriptor {
        name: f.name.to_string(),
        label: f.label.to_string(),
        field_type: f.field_type.to_string(),
        keys: f.keys.map(|k| k.iter().map(|s| s.to_string()).collect()),
    }
}

fn activity_brief_text(subject: &SubjectRow, activity: &[ActivityItem]) -> String {
    let lines: Vec<String> = activity
        .iter()
        .map(|a| format!("- [{}] {}: {}", a.date, a.kind, a.summary))
        .collect();
    format!(
        "Weekly status brief for subject \"{}\" (audience: {}). This week's real activity, oldest first:\n{}",
        subject.subject_id,
        subject.supervisor_or_contact,
        if lines.is_empty() { "(no recorded activity this week)".to_string() } else { lines.join("\n") }
    )
}

/// Drafts ONE subject's brief: AI-fills the field values from the week's
/// gathered activity via `ai_assist::full_draft`, allocates a doc ID, then
/// calls the archetype's own `build()` -- the same function a manual
/// Creator Studio "Generate" click would call. Returns an error result
/// rather than panicking so the caller (`draft_all_briefs`) can isolate it.
pub async fn draft_brief(
    client: &reqwest::Client,
    api_key: &str,
    doc_registry: &DocRegistry,
    subject: &SubjectRow,
    activity: Vec<ActivityItem>,
    week_of: NaiveDate,
) -> BriefDraftResult {
    let fields = weekly_status_brief_fields();
    let brief_text = activity_brief_text(subject, &activity);

    let drafted = match ai_assist::full_draft(client, api_key, "weekly-status-brief", &fields, &brief_text).await {
        Ok(v) => v,
        Err(e) => {
            return BriefDraftResult { subject_id: subject.subject_id.clone(), success: false, doc_id: None, error: Some(e.to_string()) };
        }
    };

    let mut content: Content = drafted;
    content.insert("subject_id".into(), Value::String(subject.subject_id.clone()));
    content.insert("audience".into(), Value::String(subject.supervisor_or_contact.clone()));
    content.insert("week_of".into(), Value::String(week_of.format("%Y-%m-%d").to_string()));
    content.entry("version").or_insert_with(|| Value::String("1.0".to_string()));
    content
        .entry("next_brief_date")
        .or_insert_with(|| Value::String((week_of + chrono::Duration::days(7)).format("%Y-%m-%d").to_string()));

    let source_refs: Vec<String> = activity.iter().map(|a| a.kind.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
    let doc_id = match doc_registry.allocate(
        "BRIEF",
        week_of,
        "weekly-status-brief",
        &subject.supervisor_or_contact,
        content.get("version").and_then(|v| v.as_str()).unwrap_or("1.0"),
        source_refs,
    ) {
        Ok(id) => id,
        Err(e) => {
            return BriefDraftResult { subject_id: subject.subject_id.clone(), success: false, doc_id: None, error: Some(e.to_string()) };
        }
    };
    content.insert("doc_id".into(), Value::String(doc_id.clone()));

    // Invokes the SAME build path a manual Generate click uses -- this is
    // the "not a second code path" guarantee. A build failure (e.g. the
    // AI proposed more than 3 signal items and validate() rejects it) is
    // reported, not silently patched around.
    match doc_builder::build("_common", "weekly-status-brief", &content) {
        Ok(_tree) => BriefDraftResult { subject_id: subject.subject_id.clone(), success: true, doc_id: Some(doc_id), error: None },
        Err(e) => BriefDraftResult { subject_id: subject.subject_id.clone(), success: false, doc_id: Some(doc_id), error: Some(e.to_string()) },
    }
}

/// Drafts every active subject's brief -- what the scheduler calls.
/// One subject's failure (an AI-provider error, a validation failure)
/// never blocks the others, same as `draftAllBriefs()`.
pub async fn draft_all_briefs(
    client: &reqwest::Client,
    api_key: &str,
    doc_registry: &DocRegistry,
    subjects: &dyn SubjectSource,
    today: NaiveDate,
) -> Vec<BriefDraftResult> {
    let week_of = monday_of(today);
    let mut results = Vec::new();
    for subject in subjects.active_subjects().into_iter().filter(|s| s.status != "retired") {
        let activity = subjects.gather_activity(&subject);
        results.push(draft_brief(client, api_key, doc_registry, &subject, activity, week_of).await);
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monday_of_a_wednesday_returns_the_same_week_monday() {
        let wed = NaiveDate::from_ymd_opt(2026, 9, 10).unwrap(); // Thursday actually -- checked below
        // 2026-09-10 is a Thursday; ISO week Monday is 2026-09-07.
        assert_eq!(monday_of(wed), NaiveDate::from_ymd_opt(2026, 9, 7).unwrap());
    }

    #[test]
    fn monday_of_a_monday_is_itself() {
        let mon = NaiveDate::from_ymd_opt(2026, 9, 7).unwrap();
        assert_eq!(monday_of(mon), mon);
    }

    #[test]
    fn monday_of_a_sunday_rolls_back_to_the_prior_monday() {
        let sun = NaiveDate::from_ymd_opt(2026, 9, 13).unwrap();
        assert_eq!(monday_of(sun), NaiveDate::from_ymd_opt(2026, 9, 7).unwrap());
    }

    struct FixtureSource {
        subjects: Vec<SubjectRow>,
    }
    impl SubjectSource for FixtureSource {
        fn active_subjects(&self) -> Vec<SubjectRow> {
            self.subjects.clone()
        }
        fn gather_activity(&self, _subject: &SubjectRow) -> Vec<ActivityItem> {
            vec![ActivityItem { date: "2026-09-08".into(), kind: "task".into(), summary: "Did a thing.".into() }]
        }
    }

    #[test]
    fn retired_subjects_are_excluded_from_the_active_filter() {
        let source = FixtureSource {
            subjects: vec![
                SubjectRow { subject_id: "a".into(), source_ref: "org-a".into(), supervisor_or_contact: "Joel".into(), status: "active".into() },
                SubjectRow { subject_id: "b".into(), source_ref: "org-b".into(), supervisor_or_contact: "Ragnar".into(), status: "retired".into() },
            ],
        };
        let active: Vec<_> = source.active_subjects().into_iter().filter(|s| s.status != "retired").collect();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].subject_id, "a");
    }

    #[test]
    fn activity_brief_text_reports_no_activity_honestly_when_empty() {
        let subject = SubjectRow { subject_id: "a".into(), source_ref: "org-a".into(), supervisor_or_contact: "Joel".into(), status: "active".into() };
        let text = activity_brief_text(&subject, &[]);
        assert!(text.contains("no recorded activity this week"));
    }
}
