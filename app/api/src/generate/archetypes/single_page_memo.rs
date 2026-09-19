//! single-page-memo -- ported from
//! `scope/lib/generate/archetypes/_common/single-page-memo.js`. Deliberately
//! minimal per upstream's own comment: flat field list, no sections, even
//! though the renderer supports them.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, get_str_array, is_missing, Content};
use crate::generate::node_tree::{document, kv_list, paragraphs, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 5] = ["to", "from", "date", "re", "body_paragraphs"];

const FIELDS: [FieldDef; 5] = [
    FieldDef { name: "to", label: "To", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "from", label: "From", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "date", label: "Date", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "re", label: "Re (subject)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef {
        name: "body_paragraphs",
        label: "Body (one paragraph per line)",
        field_type: "list",
        required: true,
        ..FIELD_DEFAULTS
    },
];

pub struct SinglePageMemo;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for SinglePageMemo {
    fn id(&self) -> &'static str {
        "single-page-memo"
    }
    fn title(&self) -> &'static str {
        "Single-Page Memo"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "re", secondary: "date" }
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
            return Err(BuildError(format!("single-page-memo: {}", v.errors.join("; "))));
        }

        let sections: Vec<Node> = vec![
            kv_list(vec![
                ("To.", get_str(content, "to").unwrap_or("")),
                ("From.", get_str(content, "from").unwrap_or("")),
                ("Date.", get_str(content, "date").unwrap_or("")),
                ("Re.", get_str(content, "re").unwrap_or("")),
            ]),
        ]
        .into_iter()
        .chain(paragraphs(get_str_array(content, "body_paragraphs").unwrap_or_default()))
        .collect();

        Ok(document(DocumentSpec {
            headline: "MEMO".to_string(),
            meta_line: None,
            sections,
            footer_note: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn content(v: serde_json::Value) -> Content {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn build_produces_a_memo_with_kv_header_and_body_paragraphs() {
        let a = SinglePageMemo;
        let c = content(json!({
            "to": "Sconl", "from": "Ada", "date": "2026-09-14", "re": "Status",
            "body_paragraphs": ["First.", "Second."],
        }));
        let tree = a.build(&c).unwrap();
        assert_eq!(tree.headline, "MEMO");
        assert_eq!(tree.sections.len(), 3); // kv_list + 2 paragraphs
    }

    #[test]
    fn build_rejects_content_missing_a_required_field() {
        let a = SinglePageMemo;
        let c = content(json!({"to": "Sconl"}));
        let err = a.build(&c).unwrap_err();
        assert!(err.0.contains("missing required field"));
    }
}
