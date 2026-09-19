//! Markdown renderer -- direct serialization of the node tree, no docx
//! round-trip (canon §7). Ported verbatim from
//! `scope/lib/generate/render-markdown.js`.

use super::node_tree::{DocumentTree, Node};
use std::fmt::Write;

fn render_node(node: &Node) -> String {
    match node {
        Node::Heading { level, text } => {
            let hashes = "#".repeat((*level as usize + 1).min(6));
            format!("{hashes} {text}")
        }
        Node::Paragraph { text } => text.clone(),
        Node::Bullets { items, .. } => items
            .iter()
            .map(|i| format!("- {i}"))
            .collect::<Vec<_>>()
            .join("\n"),
        Node::CheckedBullets { items } => items
            .iter()
            .map(|i| {
                if i.reason.is_empty() {
                    format!("- {}", i.text)
                } else {
                    format!("- {} ({})", i.text, i.reason)
                }
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Node::KvList { items } => items
            .iter()
            .map(|i| format!("**{}**  {}", i.label, i.value))
            .collect::<Vec<_>>()
            .join("\n\n"),
        Node::TruthCheck { sections_used, must_not_say } => {
            let used = format!("**Sections used:** {}", sections_used.join(", "));
            let mut parts = vec![used];
            if !must_not_say.is_empty() {
                let mut must = String::from("**What we must not say:**\n");
                let lines: Vec<String> = must_not_say
                    .iter()
                    .map(|m| {
                        if m.reason.is_empty() {
                            format!("- {}", m.claim)
                        } else {
                            format!("- {} ({})", m.claim, m.reason)
                        }
                    })
                    .collect();
                must.push_str(&lines.join("\n"));
                parts.push(must);
            }
            parts.join("\n\n")
        }
        Node::Table { header, rows } => {
            let header_row = format!("| {} |", header.join(" | "));
            let sep = format!("| {} |", header.iter().map(|_| "---").collect::<Vec<_>>().join(" | "));
            let mut out = vec![header_row, sep];
            for r in rows {
                out.push(format!("| {} |", r.join(" | ")));
            }
            out.join("\n")
        }
    }
}

pub fn render_markdown(doc: &DocumentTree) -> String {
    let mut parts = vec![format!("# {}", doc.headline)];
    if let Some(meta) = &doc.meta_line {
        parts.push(format!("*{meta}*"));
    }
    for node in &doc.sections {
        parts.push(render_node(node));
    }
    if let Some(footer) = &doc.footer_note {
        parts.push(format!("---\n\n*{footer}*"));
    }
    let mut out = String::new();
    write!(out, "{}", parts.into_iter().filter(|p| !p.is_empty()).collect::<Vec<_>>().join("\n\n")).ok();
    out.push('\n');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate::node_tree::*;

    #[test]
    fn renders_headline_meta_and_a_paragraph() {
        let doc = document(DocumentSpec {
            headline: "MEMO".into(),
            meta_line: Some("subtitle".into()),
            sections: vec![paragraph("Hello.")],
            footer_note: None,
        });
        assert_eq!(render_markdown(&doc), "# MEMO\n\n*subtitle*\n\nHello.\n");
    }

    #[test]
    fn renders_a_table() {
        let doc = document(DocumentSpec {
            headline: "T".into(),
            meta_line: None,
            sections: vec![table(vec!["A".into(), "B".into()], vec![vec!["1".into(), "2".into()]])],
            footer_note: None,
        });
        assert!(render_markdown(&doc).contains("| A | B |\n| --- | --- |\n| 1 | 2 |"));
    }
}
