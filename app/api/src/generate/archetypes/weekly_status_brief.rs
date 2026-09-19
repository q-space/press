//! weekly-status-brief -- BB26091208. New on the node-tree archetype path
//! (no JS predecessor to port here, unlike the rest of `archetypes/`):
//! iSconl's own Friday auto-draft (`scope/lib/status-brief.js`, BA26082420)
//! never went through this engine -- it built its own plain-text email body
//! straight from AI output. This archetype is the first time that data
//! shape goes through `build()`/a renderer.
//!
//! Data shape is deliberately the IMPLEMENTED flat shape
//! (`SIGNAL`/`SUBSTANCE`/`TRAJECTORY` as plain string arrays, matching
//! `status_briefs.tsv`'s real columns and `status-brief.js`'s `r.data.signal`
//! /`.substance`/`.trajectory`), not iSconl canon's nested §3.2 shape --
//! that nested version (per-item `{text, source}` objects, sub-sections)
//! was scoped in `BP26091203`/canon `D-012` but never actually built; the
//! flat shape is the one real data has always been in. Three fields this
//! archetype adds that neither shape had: `risks_blockers`, `anticipated_qa`
//! (a reasoned Q&A list -- "what will they likely ask, what's the best
//! answer"), and `next_brief_date`.
//!
//! `doc_id_type()` returns `Some("BRIEF")` -- the per-archetype code the
//! doc registry (`super::super::doc_registry`) keys its `{TYPE}_{YYYYMMDD}_W{WW}`
//! IDs on, per `BA26091105`'s scope. The registry allocates the ID and
//! writes it into `content["doc_id"]` BEFORE `build()` runs (see
//! `doc_registry.rs`'s own doc comment for why this has to happen outside
//! `build()`, not inside it) -- `build()` only ever reads it like any other
//! field, so a given `(content, version)` pair still renders byte-identical
//! every time, same as every other archetype.
//!
//! Authoring rule baked into every list-shaped field's label, not just
//! applied once by hand: max 3 items keeps a one-page brief one page.
//! `validate()` enforces the cap structurally (a build with 4+ items in a
//! capped field fails validation, the same way a missing required field
//! does) rather than leaving it as a suggestion a renderer or an AI prompt
//! could silently ignore.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_pair_array, get_str, get_str_array, is_missing, Content};
use crate::generate::node_tree::{bullets, document, heading, kv_list, table, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 7] = [
    "subject_id", "audience", "week_of", "version",
    "signal", "substance", "trajectory",
];

/// Bulleted sections this archetype caps at 3 items, per the standing
/// one-page authoring rule. `risks_blockers` and `anticipated_qa` are
/// capped too even though they're not in `REQUIRED` (both optional) --
/// an optional section that's present still has to fit the page.
const CAPPED_LIST_FIELDS: [(&str, usize); 5] = [
    ("signal", 3),
    ("substance", 3),
    ("trajectory", 3),
    ("risks_blockers", 3),
    ("anticipated_qa", 3),
];

const FIELDS: [FieldDef; 12] = [
    FieldDef { name: "subject_id", label: "Subject ID (for filename)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "audience", label: "Audience / prepared for", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "week_of", label: "Week of (Monday, ISO date)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "version", label: "Version", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "author", label: "Prepared by", field_type: "text", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "signal", label: "Signal -- headline items, max 3 (one per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "substance", label: "Substance -- delivered/in-progress detail, max 3 (one per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "trajectory", label: "Trajectory -- what's next, max 3 (one per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "risks_blockers", label: "Risks and blockers, max 3 (one per line)", field_type: "list", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "anticipated_qa", label: "Anticipated Q&A, max 3 (question | best answer, one per line)", field_type: "reasoned-list", required: false, keys: Some(&["question", "answer"]), ..FIELD_DEFAULTS },
    FieldDef { name: "next_brief_date", label: "Next brief date", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "footer_note", label: "Footer note", field_type: "text", required: false, ..FIELD_DEFAULTS },
];

pub struct WeeklyStatusBrief;

fn validate_content(content: &Content) -> ValidationResult {
    let mut errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    if is_missing(content, "next_brief_date") {
        // next_brief_date is required but not list-shaped -- REQUIRED
        // already covers it above; this branch intentionally does nothing
        // extra, kept only so a reader scanning validate() doesn't wonder
        // whether the date field was forgotten.
    }
    for (field, cap) in CAPPED_LIST_FIELDS {
        let len = content.get(field).and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
        if len > cap {
            errors.push(format!("{field}: max {cap} items (has {len})"));
        }
    }
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for WeeklyStatusBrief {
    fn id(&self) -> &'static str {
        "weekly-status-brief"
    }
    fn title(&self) -> &'static str {
        "Weekly Status Brief"
    }
    fn doc_id_type(&self) -> Option<&'static str> {
        Some("BRIEF")
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "subject_id", secondary: "week_of" }
    }
    fn fields(&self) -> &'static [FieldDef] {
        &FIELDS
    }
    fn validate(&self, content: &Content) -> ValidationResult {
        validate_content(content)
    }
    fn build(&self, content: &Content) -> Result<DocumentTree, BuildError> {
        let v = validate_content(content);
        if !v.valid {
            return Err(BuildError(format!("weekly-status-brief: {}", v.errors.join("; "))));
        }

        let author = get_str(content, "author").unwrap_or("");
        let doc_id = get_str(content, "doc_id").unwrap_or("");

        let mut info: Vec<(String, String)> = vec![
            ("Week of.".to_string(), get_str(content, "week_of").unwrap_or("").to_string()),
            ("Prepared for.".to_string(), get_str(content, "audience").unwrap_or("").to_string()),
        ];
        if !author.is_empty() {
            info.push(("Prepared by.".to_string(), author.to_string()));
        }
        if !doc_id.is_empty() {
            // Doc ID is allocated outside build() (see the module doc
            // comment) and only ever displayed here, same as every other
            // already-known field -- build() stays a pure function of
            // (content, version).
            info.push(("Doc ID.".to_string(), doc_id.to_string()));
        }

        let mut sections: Vec<Node> = vec![kv_list(info)];

        let signal = get_str_array(content, "signal").unwrap_or_default();
        sections.push(heading(2, "SIGNAL"));
        sections.push(bullets(signal));

        let substance = get_str_array(content, "substance").unwrap_or_default();
        sections.push(heading(2, "SUBSTANCE"));
        sections.push(bullets(substance));

        let trajectory = get_str_array(content, "trajectory").unwrap_or_default();
        sections.push(heading(2, "TRAJECTORY"));
        sections.push(bullets(trajectory));

        let risks = get_str_array(content, "risks_blockers").unwrap_or_default();
        if !risks.is_empty() {
            sections.push(heading(2, "RISKS AND BLOCKERS"));
            sections.push(bullets(risks));
        }

        let qa = get_pair_array(content, "anticipated_qa", "question", "answer").unwrap_or_default();
        if !qa.is_empty() {
            sections.push(heading(2, "ANTICIPATED Q&A"));
            let rows: Vec<Vec<String>> = qa.into_iter().map(|(q, a)| vec![q, a]).collect();
            sections.push(table(vec!["Question".into(), "Likely answer".into()], rows));
        }

        let title = format!(
            "Weekly Status Brief -- {}",
            get_str(content, "subject_id").unwrap_or("")
        );
        let meta_line = format!(
            "Weekly status brief | {} | week of {} | v{}{}",
            get_str(content, "audience").unwrap_or(""),
            get_str(content, "week_of").unwrap_or(""),
            get_str(content, "version").unwrap_or(""),
            if doc_id.is_empty() { String::new() } else { format!(" | {doc_id}") },
        );

        let next_brief = get_str(content, "next_brief_date").unwrap_or("");
        let user_footer = get_str(content, "footer_note").unwrap_or("");
        let footer_note = if next_brief.is_empty() && user_footer.is_empty() {
            None
        } else {
            Some(
                [format!("Next brief: {next_brief}").as_str(), user_footer]
                    .into_iter()
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join("  |  "),
            )
        };

        Ok(document(DocumentSpec { headline: title, meta_line: Some(meta_line), sections, footer_note }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn content(v: serde_json::Value) -> Content {
        v.as_object().unwrap().clone()
    }

    fn minimal() -> Content {
        content(json!({
            "subject_id": "sconl", "audience": "Joel", "week_of": "2026-09-08",
            "version": "1.0", "signal": ["Shipped X."], "substance": ["Detail on X."],
            "trajectory": ["Ship Y next."], "next_brief_date": "2026-09-15",
        }))
    }

    #[test]
    fn build_produces_signal_substance_trajectory_sections_in_order() {
        let tree = WeeklyStatusBrief.build(&minimal()).unwrap();
        let headings: Vec<&str> = tree
            .sections
            .iter()
            .filter_map(|n| match n {
                Node::Heading { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(headings, vec!["SIGNAL", "SUBSTANCE", "TRAJECTORY"]);
    }

    #[test]
    fn build_adds_risks_and_qa_sections_only_when_present() {
        let mut c = minimal();
        c.insert("risks_blockers".into(), json!(["Blocker A."]));
        c.insert("anticipated_qa".into(), json!([{"question": "Q1?", "answer": "A1."}]));
        let tree = WeeklyStatusBrief.build(&c).unwrap();
        let headings: Vec<&str> = tree
            .sections
            .iter()
            .filter_map(|n| match n {
                Node::Heading { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(headings, vec!["SIGNAL", "SUBSTANCE", "TRAJECTORY", "RISKS AND BLOCKERS", "ANTICIPATED Q&A"]);
    }

    #[test]
    fn build_carries_the_actual_answer_text_into_the_qa_table_not_just_the_question() {
        // Regression: get_pair_array used to read a hardcoded "reason" key
        // for the second column, so an "answer"-keyed pair silently rendered
        // an empty second cell.
        let mut c = minimal();
        c.insert("anticipated_qa".into(), json!([{"question": "Q1?", "answer": "A1."}]));
        let tree = WeeklyStatusBrief.build(&c).unwrap();
        let table_rows = tree.sections.iter().find_map(|n| match n {
            Node::Table { rows, .. } => Some(rows.clone()),
            _ => None,
        }).unwrap();
        assert_eq!(table_rows, vec![vec!["Q1?".to_string(), "A1.".to_string()]]);
    }

    #[test]
    fn build_rejects_missing_required_field() {
        let c = content(json!({"subject_id": "s1"}));
        assert!(WeeklyStatusBrief.build(&c).is_err());
    }

    #[test]
    fn build_rejects_more_than_three_signal_items() {
        let mut c = minimal();
        c.insert("signal".into(), json!(["a", "b", "c", "d"]));
        let err = WeeklyStatusBrief.build(&c).unwrap_err();
        assert!(err.0.contains("signal: max 3 items (has 4)"));
    }

    #[test]
    fn build_includes_doc_id_in_info_kv_and_meta_line_when_present() {
        let mut c = minimal();
        c.insert("doc_id".into(), json!("BRIEF_20260908_W37"));
        let tree = WeeklyStatusBrief.build(&c).unwrap();
        assert!(tree.meta_line.unwrap().contains("BRIEF_20260908_W37"));
        let kv = tree.sections.iter().find_map(|n| match n {
            Node::KvList { items } => Some(items.clone()),
            _ => None,
        }).unwrap();
        assert!(kv.iter().any(|i| i.label == "Doc ID." && i.value == "BRIEF_20260908_W37"));
    }

    #[test]
    fn build_sets_next_brief_footer() {
        let tree = WeeklyStatusBrief.build(&minimal()).unwrap();
        assert_eq!(tree.footer_note.unwrap(), "Next brief: 2026-09-15");
    }
}
