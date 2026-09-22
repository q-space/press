//! formal-letter -- ported from
//! `scope/lib/generate/archetypes/_common/formal-letter.js`. Sectioned
//! (Header/Body/Closing) purely as wizard-UI metadata (BB26091204's
//! concern) -- the rendered document itself is flat.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, get_str_array, is_missing, Content};
use crate::generate::node_tree::{document, paragraph, paragraphs, kv_list, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 7] =
    ["sender_name", "recipient_name", "date", "salutation", "body_paragraphs", "closing", "signatory_name"];

const FIELDS: [FieldDef; 10] = [
    FieldDef { name: "sender_name", label: "Sender name", field_type: "text", required: true, section: Some("Header"), keys: None },
    FieldDef { name: "sender_address", label: "Sender address", field_type: "text", required: false, section: Some("Header"), keys: None },
    FieldDef { name: "recipient_name", label: "Recipient name", field_type: "text", required: true, section: Some("Header"), keys: None },
    FieldDef { name: "recipient_address", label: "Recipient address", field_type: "text", required: false, section: Some("Header"), keys: None },
    FieldDef { name: "date", label: "Date", field_type: "text", required: true, section: Some("Header"), keys: None },
    FieldDef { name: "subject_line", label: "Subject line (optional, \"Re: ...\")", field_type: "text", required: false, section: Some("Header"), keys: None },
    FieldDef { name: "salutation", label: "Salutation (e.g. \"Dear Mr Smith\")", field_type: "text", required: true, section: Some("Body"), keys: None },
    FieldDef { name: "body_paragraphs", label: "Body (one paragraph per line)", field_type: "list", required: true, section: Some("Body"), keys: None },
    FieldDef { name: "closing", label: "Closing (e.g. \"Yours sincerely\")", field_type: "text", required: true, section: Some("Closing"), keys: None },
    FieldDef { name: "signatory_name", label: "Signatory name", field_type: "text", required: true, section: Some("Closing"), keys: None },
];

pub struct FormalLetter;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for FormalLetter {
    fn id(&self) -> &'static str {
        "formal-letter"
    }
    fn title(&self) -> &'static str {
        "Formal Letter"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "recipient_name", secondary: "date" }
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
            return Err(BuildError(format!("formal-letter: {}", v.errors.join("; "))));
        }

        let from = [get_str(content, "sender_name"), get_str(content, "sender_address")]
            .into_iter()
            .flatten()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(", ");
        let to = [get_str(content, "recipient_name"), get_str(content, "recipient_address")]
            .into_iter()
            .flatten()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(", ");

        let mut sections: Vec<Node> = vec![
            kv_list(vec![
                ("From.", from),
                ("To.", to),
                ("Date.", get_str(content, "date").unwrap_or("").to_string()),
            ]),
            paragraph(format!("{},", get_str(content, "salutation").unwrap_or(""))),
        ];
        sections.extend(paragraphs(get_str_array(content, "body_paragraphs").unwrap_or_default()));
        sections.push(paragraph(format!("{},", get_str(content, "closing").unwrap_or(""))));
        sections.push(paragraph(get_str(content, "signatory_name").unwrap_or("")));

        let headline = get_str(content, "subject_line").filter(|s| !s.is_empty()).unwrap_or("Letter").to_string();

        Ok(document(DocumentSpec { headline, meta_line: None, sections, footer_note: None }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_joins_sender_name_and_address_in_the_kv_header() {
        let c: Content = json!({
            "sender_name": "Ada", "sender_address": "12 Main St",
            "recipient_name": "Bob", "date": "2026-09-14",
            "salutation": "Dear Bob", "body_paragraphs": ["Body."],
            "closing": "Regards", "signatory_name": "Ada",
        })
        .as_object()
        .unwrap()
        .clone();
        let tree = FormalLetter.build(&c).unwrap();
        match &tree.sections[0] {
            Node::KvList { items } => assert_eq!(items[0].value, "Ada, 12 Main St"),
            _ => panic!("expected kv_list first"),
        }
        assert_eq!(tree.headline, "Letter");
    }
}
