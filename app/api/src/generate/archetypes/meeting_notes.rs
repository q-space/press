//! meeting-notes -- ported from
//! `scope/lib/generate/archetypes/_common/meeting-notes.js`.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, get_str_array, is_missing, Content};
use crate::generate::node_tree::{bullets, document, heading, kv_list, table, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 4] = ["meeting_title", "date", "attendees", "decisions"];

const FIELDS: [FieldDef; 7] = [
    FieldDef { name: "meeting_title", label: "Meeting title", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "date", label: "Date", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "attendees", label: "Attendees (one per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "agenda", label: "Agenda (one per line)", field_type: "list", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "decisions", label: "Decisions (one per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "action_items", label: "Action items (action | owner, one per line)", field_type: "reasoned-list", required: false, keys: Some(&["action", "owner"]), ..FIELD_DEFAULTS },
    FieldDef { name: "next_meeting", label: "Next meeting", field_type: "text", required: false, ..FIELD_DEFAULTS },
];

pub struct MeetingNotes;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for MeetingNotes {
    fn id(&self) -> &'static str {
        "meeting-notes"
    }
    fn title(&self) -> &'static str {
        "Meeting Notes"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "meeting_title", secondary: "date" }
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
            return Err(BuildError(format!("meeting-notes: {}", v.errors.join("; "))));
        }

        let attendees = get_str_array(content, "attendees").unwrap_or_default();
        let agenda = get_str_array(content, "agenda").unwrap_or_default();
        let decisions = get_str_array(content, "decisions").unwrap_or_default();
        let action_items = content.get("action_items").and_then(|v| v.as_array()).cloned().unwrap_or_default();

        let mut sections: Vec<Node> = vec![kv_list(vec![("Attendees.", attendees.join(", "))])];

        if !agenda.is_empty() {
            sections.push(heading(2, "Agenda"));
            sections.push(bullets(agenda));
        }

        sections.push(heading(2, "Decisions"));
        sections.push(bullets(decisions));

        if !action_items.is_empty() {
            let rows: Vec<Vec<String>> = action_items
                .iter()
                .map(|a| {
                    let obj = a.as_object();
                    let action = obj
                        .and_then(|m| m.get("action").or_else(|| m.get("option")))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let owner = obj
                        .and_then(|m| m.get("owner").or_else(|| m.get("detail")))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    vec![action.to_string(), owner.to_string()]
                })
                .collect();
            sections.push(heading(2, "Action Items"));
            sections.push(table(vec!["Action".into(), "Owner".into()], rows));
        }

        if let Some(next) = get_str(content, "next_meeting").filter(|s| !s.is_empty()) {
            sections.push(heading(2, "Next Meeting"));
            sections.push(kv_list(vec![("When.", next.to_string())]));
        }

        Ok(document(DocumentSpec {
            headline: get_str(content, "meeting_title").unwrap_or("").to_string(),
            meta_line: Some(get_str(content, "date").unwrap_or("").to_string()),
            sections,
            footer_note: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_skips_optional_sections_when_absent() {
        let c: Content = json!({
            "meeting_title": "Sync", "date": "2026-09-14",
            "attendees": ["Ada", "Bob"], "decisions": ["Ship it."],
        })
        .as_object()
        .unwrap()
        .clone();
        let tree = MeetingNotes.build(&c).unwrap();
        // Attendees kv_list + Decisions heading + bullets = 3 sections; no agenda/action items/next meeting.
        assert_eq!(tree.sections.len(), 3);
    }
}
