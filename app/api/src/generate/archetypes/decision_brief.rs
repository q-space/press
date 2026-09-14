//! decision-brief -- ported from
//! `scope/lib/generate/archetypes/_common/decision-brief.js`. Deliberately
//! narrow: one decision, one recommendation, one approval slot.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, is_missing, Content};
use crate::generate::node_tree::{bullets, document, heading, kv_list, paragraphs, table, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 10] = [
    "subject_id", "audience", "title", "date_readable", "version",
    "decision_ask", "context_paragraphs", "recommendation", "approval_name", "approval_role",
];

const FIELDS: [FieldDef; 14] = [
    FieldDef { name: "subject_id", label: "Subject ID (for filename)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "audience", label: "Audience (for filename, e.g. \"alex\")", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "title", label: "Title", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "date_readable", label: "Date (readable)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "version", label: "Version", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "author", label: "Author", field_type: "text", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "decision_ask", label: "Decision requested", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "context_paragraphs", label: "Context (one paragraph per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "options", label: "Options (option | detail, one per line)", field_type: "reasoned-list", required: false, keys: Some(&["option", "detail"]), ..FIELD_DEFAULTS },
    FieldDef { name: "recommendation", label: "Recommendation", field_type: "textarea", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "risks", label: "Risks and considerations (one per line)", field_type: "list", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "approval_name", label: "Approver name", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "approval_role", label: "Approver role", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "footer_note", label: "Footer note", field_type: "text", required: false, ..FIELD_DEFAULTS },
];

pub struct DecisionBrief;

fn validate_content(content: &Content) -> ValidationResult {
    // JS's validate() here checks only undefined/null/'' -- NOT the
    // empty-array case is_missing() also covers for other archetypes.
    // None of decision-brief's REQUIRED fields are arrays (context_paragraphs
    // is the only array-typed required field, and JS's own REQUIRED list
    // includes it, checked only against undefined/null/'' there too -- an
    // empty array passes JS's check). Preserved as a real behavioral
    // quirk, not silently "fixed": is_missing_strict() below mirrors it.
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing_strict(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

fn is_missing_strict(content: &Content, field: &str) -> bool {
    match content.get(field) {
        None => true,
        Some(serde_json::Value::Null) => true,
        Some(serde_json::Value::String(s)) => s.is_empty(),
        Some(_) => false,
    }
}

impl Archetype for DecisionBrief {
    fn id(&self) -> &'static str {
        "decision-brief"
    }
    fn title(&self) -> &'static str {
        "Decision Brief"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "subject_id", secondary: "audience" }
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
            return Err(BuildError(format!("decision-brief: {}", v.errors.join("; "))));
        }

        let author = get_str(content, "author").unwrap_or("");
        let options = content.get("options").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let risks = content.get("risks").and_then(|v| v.as_array()).cloned().unwrap_or_default();

        let mut sections: Vec<Node> = vec![
            kv_list(vec![("Decision requested.", get_str(content, "decision_ask").unwrap_or(""))]),
            heading(2, "1.   CONTEXT"),
        ];
        let context_paragraphs = content
            .get("context_paragraphs")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|x| x.as_str().map(String::from)).collect::<Vec<_>>())
            .unwrap_or_default();
        sections.extend(paragraphs(context_paragraphs));

        if !options.is_empty() {
            let rows: Vec<Vec<String>> = options
                .iter()
                .map(|o| {
                    let obj = o.as_object();
                    let option = obj
                        .and_then(|m| m.get("option").or_else(|| m.get("label")))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let detail = obj
                        .and_then(|m| m.get("detail").or_else(|| m.get("note")))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    vec![option.to_string(), detail.to_string()]
                })
                .collect();
            sections.push(heading(2, "2.   OPTIONS CONSIDERED"));
            sections.push(table(vec!["Option".into(), "Detail".into()], rows));
        }

        sections.push(heading(2, if options.is_empty() { "2.   RECOMMENDATION" } else { "3.   RECOMMENDATION" }));
        sections.extend(paragraphs(vec![get_str(content, "recommendation").unwrap_or("").to_string()]));

        if !risks.is_empty() {
            let risk_strs: Vec<String> = risks.iter().filter_map(|r| r.as_str().map(String::from)).collect();
            let n = if options.is_empty() { "3" } else { "4" };
            sections.push(heading(2, format!("{n}.   RISKS AND CONSIDERATIONS")));
            sections.push(bullets(risk_strs));
        }

        sections.push(heading(2, "APPROVAL"));
        sections.push(table(
            vec!["Approver".into(), "Role".into(), "Decision".into(), "Date".into()],
            vec![vec![
                get_str(content, "approval_name").unwrap_or("").to_string(),
                get_str(content, "approval_role").unwrap_or("").to_string(),
                String::new(),
                String::new(),
            ]],
        ));

        let title = get_str(content, "title").unwrap_or("").to_string();
        let meta_line = format!(
            "Decision brief | {} | {} | v{} | {}",
            get_str(content, "audience").unwrap_or(""),
            get_str(content, "date_readable").unwrap_or(""),
            get_str(content, "version").unwrap_or(""),
            author,
        );
        let footer_note = get_str(content, "footer_note").map(String::from);

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
            "subject_id": "s1", "audience": "alex", "title": "Ship it?",
            "date_readable": "14 Sep 2026", "version": "1.0",
            "decision_ask": "Ship v2 now?", "context_paragraphs": ["Context."],
            "recommendation": "Ship it.", "approval_name": "Sconl", "approval_role": "Owner",
        }))
    }

    #[test]
    fn build_without_options_or_risks_numbers_recommendation_as_section_2() {
        let tree = DecisionBrief.build(&minimal()).unwrap();
        let headings: Vec<&str> = tree
            .sections
            .iter()
            .filter_map(|n| match n {
                Node::Heading { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(headings, vec!["1.   CONTEXT", "2.   RECOMMENDATION", "APPROVAL"]);
    }

    #[test]
    fn build_with_options_renumbers_recommendation_to_section_3() {
        let mut c = minimal();
        c.insert("options".into(), json!([{"option": "A", "detail": "d"}]));
        let tree = DecisionBrief.build(&c).unwrap();
        let headings: Vec<&str> = tree
            .sections
            .iter()
            .filter_map(|n| match n {
                Node::Heading { text, .. } => Some(text.as_str()),
                _ => None,
            })
            .collect();
        assert_eq!(headings, vec!["1.   CONTEXT", "2.   OPTIONS CONSIDERED", "3.   RECOMMENDATION", "APPROVAL"]);
    }

    #[test]
    fn build_rejects_missing_required_field() {
        let c = content(json!({"subject_id": "s1"}));
        assert!(DecisionBrief.build(&c).is_err());
    }
}
