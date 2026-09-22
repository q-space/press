//! email -- ported from `scope/lib/generate/archetypes/_common/email.js`.
//! `layout: "header-block"` renders to/cc/subject as a compact header row
//! in the input form, per BA26081810's UX hint (this is metadata read by
//! Qpress's Creator Studio wizard, BB26091204 -- it has no effect here).

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, get_str_array, is_missing, Content};
use crate::generate::node_tree::{document, paragraphs, DocumentSpec, DocumentTree};

const REQUIRED: [&str; 3] = ["to", "subject", "body_paragraphs"];

const FIELDS: [FieldDef; 5] = [
    FieldDef { name: "to", label: "To", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "cc", label: "Cc", field_type: "text", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "subject", label: "Subject", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "body_paragraphs", label: "Body (one paragraph per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "signature", label: "Signature / sign-off", field_type: "text", required: false, ..FIELD_DEFAULTS },
];

pub struct Email;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for Email {
    fn id(&self) -> &'static str {
        "email"
    }
    fn title(&self) -> &'static str {
        "Email"
    }
    fn layout(&self) -> Option<&'static str> {
        Some("header-block")
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "to", secondary: "subject" }
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
            return Err(BuildError(format!("email: {}", v.errors.join("; "))));
        }

        let mut meta_parts = vec![format!("To: {}", get_str(content, "to").unwrap_or(""))];
        if let Some(cc) = get_str(content, "cc") {
            if !cc.is_empty() {
                meta_parts.push(format!("Cc: {cc}"));
            }
        }

        Ok(document(DocumentSpec {
            headline: get_str(content, "subject").unwrap_or("").to_string(),
            meta_line: Some(meta_parts.join("  |  ")),
            sections: paragraphs(get_str_array(content, "body_paragraphs").unwrap_or_default()),
            footer_note: get_str(content, "signature").filter(|s| !s.is_empty()).map(String::from),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_includes_cc_only_when_present() {
        let c: Content = json!({"to": "a@x.com", "subject": "Hi", "body_paragraphs": ["Body."]})
            .as_object()
            .unwrap()
            .clone();
        let tree = Email.build(&c).unwrap();
        assert_eq!(tree.meta_line.unwrap(), "To: a@x.com");

        let mut c2 = c;
        c2.insert("cc".into(), json!("b@x.com"));
        let tree2 = Email.build(&c2).unwrap();
        assert_eq!(tree2.meta_line.unwrap(), "To: a@x.com  |  Cc: b@x.com");
    }
}
