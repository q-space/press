//! proposal -- ported from
//! `scope/lib/generate/archetypes/_common/proposal.js`. Problem/solution/
//! timeline/cost/next-steps shape.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, get_str_array, is_missing, Content};
use crate::generate::node_tree::{bullets, document, heading, kv_list, paragraphs, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 6] = ["title", "client", "date", "problem_statement", "proposed_solution", "next_steps"];

const FIELDS: [FieldDef; 8] = [
    FieldDef { name: "title", label: "Title", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "client", label: "Prepared for (client)", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "date", label: "Date", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "problem_statement", label: "Problem", field_type: "textarea", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "proposed_solution", label: "Proposed solution", field_type: "textarea", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "timeline", label: "Timeline (one milestone per line)", field_type: "list", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "cost", label: "Cost", field_type: "textarea", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "next_steps", label: "Next steps (one per line)", field_type: "list", required: true, ..FIELD_DEFAULTS },
];

pub struct Proposal;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for Proposal {
    fn id(&self) -> &'static str {
        "proposal"
    }
    fn title(&self) -> &'static str {
        "Proposal"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "client", secondary: "title" }
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
            return Err(BuildError(format!("proposal: {}", v.errors.join("; "))));
        }

        let mut sections: Vec<Node> = vec![
            kv_list(vec![("Prepared for.", get_str(content, "client").unwrap_or(""))]),
            heading(2, "Problem"),
        ];
        sections.extend(paragraphs(vec![get_str(content, "problem_statement").unwrap_or("").to_string()]));
        sections.push(heading(2, "Proposed Solution"));
        sections.extend(paragraphs(vec![get_str(content, "proposed_solution").unwrap_or("").to_string()]));

        let timeline = get_str_array(content, "timeline").unwrap_or_default();
        if !timeline.is_empty() {
            sections.push(heading(2, "Timeline"));
            sections.push(bullets(timeline));
        }

        if let Some(cost) = get_str(content, "cost").filter(|s| !s.is_empty()) {
            sections.push(heading(2, "Cost"));
            sections.extend(paragraphs(vec![cost.to_string()]));
        }

        sections.push(heading(2, "Next Steps"));
        sections.push(bullets(get_str_array(content, "next_steps").unwrap_or_default()));

        Ok(document(DocumentSpec {
            headline: get_str(content, "title").unwrap_or("").to_string(),
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
    fn build_omits_timeline_and_cost_when_absent() {
        let c: Content = json!({
            "title": "New site", "client": "Acme", "date": "2026-09-14",
            "problem_statement": "Slow site.", "proposed_solution": "Rebuild it.",
            "next_steps": ["Kickoff call."],
        })
        .as_object()
        .unwrap()
        .clone();
        let tree = Proposal.build(&c).unwrap();
        // Prepared-for + Problem heading/para + Solution heading/para + Next Steps heading/bullets = 7
        assert_eq!(tree.sections.len(), 7);
    }
}
