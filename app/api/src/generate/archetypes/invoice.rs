//! invoice -- ported from `scope/lib/generate/archetypes/_common/invoice.js`.
//! Deliberately simple (line items + totals), a document, not a ledger.

use crate::generate::archetype::{Archetype, BuildError, FieldDef, FilenameFields, ValidationResult, FIELD_DEFAULTS};
use crate::generate::content::{get_str, is_missing, Content};
use crate::generate::node_tree::{document, heading, kv_list, table, DocumentSpec, DocumentTree, Node};

const REQUIRED: [&str; 5] = ["invoice_number", "date", "bill_to", "line_items", "total"];

const FIELDS: [FieldDef; 8] = [
    FieldDef { name: "invoice_number", label: "Invoice number", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "date", label: "Date", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "bill_to", label: "Bill to", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "line_items", label: "Line items (description | amount, one per line)", field_type: "reasoned-list", required: true, keys: Some(&["description", "amount"]), ..FIELD_DEFAULTS },
    FieldDef { name: "subtotal", label: "Subtotal", field_type: "text", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "tax", label: "Tax", field_type: "text", required: false, ..FIELD_DEFAULTS },
    FieldDef { name: "total", label: "Total", field_type: "text", required: true, ..FIELD_DEFAULTS },
    FieldDef { name: "payment_terms", label: "Payment terms", field_type: "text", required: false, ..FIELD_DEFAULTS },
];

pub struct Invoice;

fn validate_content(content: &Content) -> ValidationResult {
    let errors: Vec<String> = REQUIRED
        .iter()
        .filter(|f| is_missing(content, f))
        .map(|f| format!("missing required field: {f}"))
        .collect();
    ValidationResult { valid: errors.is_empty(), errors }
}

impl Archetype for Invoice {
    fn id(&self) -> &'static str {
        "invoice"
    }
    fn title(&self) -> &'static str {
        "Invoice"
    }
    fn filename_fields(&self) -> FilenameFields {
        FilenameFields { primary: "invoice_number", secondary: "bill_to" }
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
            return Err(BuildError(format!("invoice: {}", v.errors.join("; "))));
        }

        let line_items = content.get("line_items").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        let rows: Vec<Vec<String>> = line_items
            .iter()
            .map(|i| {
                let obj = i.as_object();
                let desc = obj
                    .and_then(|m| m.get("description").or_else(|| m.get("option")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let amount = obj
                    .and_then(|m| m.get("amount").or_else(|| m.get("detail")))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                vec![desc.to_string(), amount.to_string()]
            })
            .collect();

        let mut totals: Vec<(String, String)> = vec![];
        if let Some(subtotal) = get_str(content, "subtotal").filter(|s| !s.is_empty()) {
            totals.push(("Subtotal.".to_string(), subtotal.to_string()));
        }
        if let Some(tax) = get_str(content, "tax").filter(|s| !s.is_empty()) {
            totals.push(("Tax.".to_string(), tax.to_string()));
        }
        totals.push(("Total.".to_string(), get_str(content, "total").unwrap_or("").to_string()));

        let sections: Vec<Node> = vec![
            kv_list(vec![
                ("Invoice #.".to_string(), get_str(content, "invoice_number").unwrap_or("").to_string()),
                ("Date.".to_string(), get_str(content, "date").unwrap_or("").to_string()),
                ("Bill to.".to_string(), get_str(content, "bill_to").unwrap_or("").to_string()),
            ]),
            heading(2, "Line Items"),
            table(vec!["Description".into(), "Amount".into()], rows),
            kv_list(totals),
        ];

        Ok(document(DocumentSpec {
            headline: format!("Invoice {}", get_str(content, "invoice_number").unwrap_or("")),
            meta_line: Some(get_str(content, "date").unwrap_or("").to_string()),
            sections,
            footer_note: get_str(content, "payment_terms").filter(|s| !s.is_empty()).map(String::from),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn build_includes_subtotal_and_tax_only_when_present() {
        let c: Content = json!({
            "invoice_number": "INV-1", "date": "2026-09-14", "bill_to": "Acme",
            "line_items": [{"description": "Design", "amount": "500"}],
            "total": "500",
        })
        .as_object()
        .unwrap()
        .clone();
        let tree = Invoice.build(&c).unwrap();
        match &tree.sections[3] {
            Node::KvList { items } => assert_eq!(items.len(), 1),
            _ => panic!("expected kv_list totals last"),
        }
        assert_eq!(tree.headline, "Invoice INV-1");
    }
}
