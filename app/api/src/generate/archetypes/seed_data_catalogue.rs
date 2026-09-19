//! seed-data-catalogue -- ported from
//! `scope/lib/generate/archetypes/_common/seed-data-catalogue.js`. A
//! tabular reference document; namespaced `_common` since "here is a
//! table of sample data" isn't specific to any one engagement.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, get_str_array, Content};
use crate::generate::node_tree::{document, heading, paragraphs, table, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 6] = ["catalogue_id", "scope_name", "title", "date_readable", "version", "entries"];

const FIELDS: [FieldDef; 9] = [
    FieldDef { name: "catalogue_id", label: "Catalogue ID (for filename)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "scope_name", label: "Scope name (for filename)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "title", label: "Title", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "date_readable", label: "Date (readable)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "version", label: "Version", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "author", label: "Author", field_type: "text", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "description_paragraphs", label: "Scope description (one paragraph per line)", field_type: "list", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "entries", label: "Seed data rows (category | field | example value | notes, one per line)", field_type: "table-list", required: true, keys: Some(&["category", "field", "example_value", "notes"]), ..FIELD_DEFAULTS },
    FieldDef { name: "footer_note", label: "Footer note", field_type: "text", required: false, ..FIELD_DEFAULTS },
];

pub struct SeedDataCatalogue;

// JS's own validate() checks REQUIRED against undefined/null/'' only (not
// the empty-array rule content.rs's is_missing() applies elsewhere), then
// separately special-cases `entries` with its own "at least one row"
// message -- preserved as two distinct checks, not merged, since the
// error message differs.
fn validate_content(content: &Content) -> ValidationResult {
    let mut errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| match content.get(**f) {
            None => true,
            Some(serde_json::Value::Null) => true,
            Some(serde_json::Value::String(s)) => s.is_empty(),
            Some(_) => false,
        })
        .map(|f| format!("missing required field: {f}"))
        .collect();
    if let Some(entries) = content.get("entries").and_then(|v| v.as_array()) {
        if entries.is_empty() {
            errors.push("entries must have at least one row".to_string());
        }
    }
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for SeedDataCatalogue {
    fn id(&self) -> &'static str {
        "seed-data-catalogue"
    }
    fn title(&self) -> &'static str {
        "Seed-Data Catalogue"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "catalogue_id", secondary: "scope_name" }
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
            return Err(BuildError(format!("seed-data-catalogue: {}", v.errors.join("; "))));
        }

        let author = get_str(content, "author").unwrap_or("");
        let mut sections: Vec<Node> = vec![];

        let description = get_str_array(content, "description_paragraphs").unwrap_or_default();
        if !description.is_empty() {
            sections.push(heading(2, "SCOPE"));
            sections.extend(paragraphs(description));
        }

        let entries = content.get("entries").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let rows: Vec<Vec<String>> = entries
            .iter()
            .map(|e| {
                let obj = e.as_object();
                let get = |k: &str| obj.and_then(|m| m.get(k)).and_then(|v| v.as_str()).unwrap_or("").to_string();
                let example = obj
                    .and_then(|m| m.get("example_value").or_else(|| m.get("example")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                vec![get("category"), get("field"), example, get("notes")]
            })
            .collect();
        sections.push(heading(2, "SEED DATA"));
        sections.push(table(
            vec!["Category".into(), "Field".into(), "Example value".into(), "Notes".into()],
            rows,
        ));

        let meta_line = format!(
            "Seed-data catalogue | {} | {} | v{} | {}",
            get_str(content, "scope_name").unwrap_or(""),
            get_str(content, "date_readable").unwrap_or(""),
            get_str(content, "version").unwrap_or(""),
            author,
        );

        Ok(document(DocumentSpec {
            headline: get_str(content, "title").unwrap_or("").to_string(),
            meta_line: Some(meta_line),
            sections,
            footer_note: get_str(content, "footer_note").filter(|s| !s.is_empty()).map(String::from),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_rejects_an_empty_entries_array_with_its_own_message() {
        let c: Content = json!({
            "catalogue_id": "c1", "scope_name": "s1", "title": "T",
            "date_readable": "14 Sep 2026", "version": "1.0", "entries": [],
        })
        .as_object()
        .unwrap()
        .clone();
        let err = SeedDataCatalogue.build(&c).unwrap_err();
        assert!(err.0.contains("entries must have at least one row"));
    }
}
