//! multipage-report -- ported from
//! `scope/lib/generate/archetypes/_common/multipage-report.js`. Each body
//! section's `type` (default "paragraph") selects which node-tree
//! constructor it dispatches to: "table" expects `body: {header, rows}`,
//! "bullets" expects `body: string[]`, anything else treats `body` as a
//! plain string paragraph.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, is_missing, Content};
use crate::generate::node_tree::{bullets, document, heading, paragraph, paragraphs, table, DocumentSpec, DocumentTree, Node};
use serde_json::Value;

const REQUIRED: [&str; 5] = ["title", "date_readable", "executive_summary", "report_sections", "conclusion"];

const FIELDS: [FieldDef; 7] = [
    FieldDef { name: "title", label: "Title", field_type: "text", required: true, section: Some("Cover"), keys: None },
    FieldDef { name: "date_readable", label: "Date (readable)", field_type: "text", required: true, section: Some("Cover"), keys: None },
    FieldDef { name: "author", label: "Author", field_type: "text", required: false, section: Some("Cover"), keys: None },
    FieldDef { name: "executive_summary", label: "Executive summary", field_type: "textarea", required: true, section: Some("Summary"), keys: None },
    FieldDef {
        name: "report_sections",
        label: "Body sections (heading | type | body -- type is paragraph/table/bullets, default paragraph)",
        field_type: "reasoned-list",
        required: true,
        section: Some("Body"),
        keys: Some(&["heading", "type", "body"]),
    },
    FieldDef { name: "conclusion", label: "Conclusion", field_type: "textarea", required: true, section: Some("Conclusion"), keys: None },
    FieldDef { name: "footer_note", label: "Footer note", field_type: "text", required: false, section: Some("Conclusion"), keys: None },
];

pub struct MultipageReport;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

fn section_node(s: &Value) -> Node {
    let obj = s.as_object();
    let body = obj.and_then(|m| m.get("body").or_else(|| m.get("detail")));
    let section_type = obj
        .and_then(|m| m.get("type"))
        .and_then(|v| v.as_str())
        .unwrap_or("paragraph");

    match section_type {
        "table" => {
            let header = body
                .and_then(|b| b.get("header"))
                .and_then(|v| v.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default();
            let rows = body
                .and_then(|b| b.get("rows"))
                .and_then(|v| v.as_array())
                .map(|rows| {
                    rows.iter()
                        .filter_map(|r| r.as_array())
                        .map(|r| r.iter().filter_map(|c| c.as_str().map(String::from)).collect())
                        .collect()
                })
                .unwrap_or_default();
            table(header, rows)
        }
        "bullets" => {
            let items: Vec<String> = body
                .and_then(|b| b.as_array())
                .map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect())
                .unwrap_or_default();
            bullets(items)
        }
        _ => paragraph(body.and_then(|b| b.as_str()).unwrap_or("")),
    }
}

impl Archetype for MultipageReport {
    fn id(&self) -> &'static str {
        "multipage-report"
    }
    fn title(&self) -> &'static str {
        "Multipage Report"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "title", secondary: "date_readable" }
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
            return Err(BuildError(format!("multipage-report: {}", v.errors.join("; "))));
        }

        let mut sections: Vec<Node> = vec![heading(2, "Executive Summary")];
        sections.extend(paragraphs(vec![get_str(content, "executive_summary").unwrap_or("").to_string()]));

        let report_sections = content.get("report_sections").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        for s in &report_sections {
            let obj = s.as_object();
            let label = obj
                .and_then(|m| m.get("heading").or_else(|| m.get("label")))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            sections.push(heading(2, label));
            sections.push(section_node(s));
        }

        sections.push(heading(2, "Conclusion"));
        sections.extend(paragraphs(vec![get_str(content, "conclusion").unwrap_or("").to_string()]));

        let meta_line = [get_str(content, "date_readable"), get_str(content, "author")]
            .into_iter()
            .flatten()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" | ");

        Ok(document(DocumentSpec {
            headline: get_str(content, "title").unwrap_or("").to_string(),
            meta_line: if meta_line.is_empty() { None } else { Some(meta_line) },
            sections,
            footer_note: get_str(content, "footer_note").filter(|s| !s.is_empty()).map(String::from),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn minimal(report_sections: Value) -> Content {
        json!({
            "title": "State of Q3", "date_readable": "14 Sep 2026",
            "executive_summary": "All good.", "report_sections": report_sections,
            "conclusion": "Ship it.",
        })
        .as_object()
        .unwrap()
        .clone()
    }

    #[test]
    fn table_type_section_reads_header_and_rows() {
        let c = minimal(json!([{
            "heading": "Numbers", "type": "table",
            "body": {"header": ["A", "B"], "rows": [["1", "2"]]},
        }]));
        let tree = MultipageReport.build(&c).unwrap();
        match &tree.sections[3] {
            Node::Table { header, rows } => {
                assert_eq!(header, &vec!["A".to_string(), "B".to_string()]);
                assert_eq!(rows, &vec![vec!["1".to_string(), "2".to_string()]]);
            }
            other => panic!("expected table node, got {other:?}"),
        }
    }

    #[test]
    fn bullets_type_section_reads_a_string_array() {
        let c = minimal(json!([{"heading": "Risks", "type": "bullets", "body": ["r1", "r2"]}]));
        let tree = MultipageReport.build(&c).unwrap();
        match &tree.sections[3] {
            Node::Bullets { items, .. } => assert_eq!(items, &vec!["r1".to_string(), "r2".to_string()]),
            other => panic!("expected bullets node, got {other:?}"),
        }
    }

    #[test]
    fn default_type_treats_body_as_a_paragraph_string() {
        let c = minimal(json!([{"heading": "Context", "body": "Plain text."}]));
        let tree = MultipageReport.build(&c).unwrap();
        match &tree.sections[3] {
            Node::Paragraph { text } => assert_eq!(text, "Plain text."),
            other => panic!("expected paragraph node, got {other:?}"),
        }
    }
}
