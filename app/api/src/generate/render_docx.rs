//! docx renderer -- builds a .docx PROGRAMMATICALLY with `docx-rs`, never by
//! templating text into an existing Word file (canon §7). Ported from
//! `scope/lib/generate/render-docx.js`, same paragraph/run shape, same
//! lead-in-**bold** bullet convention.

use super::node_tree::{DocumentTree, Node};
use super::style::STYLE;
use docx_rs::{
    AlignmentType, Docx, Paragraph, Run, RunFonts, Table, TableAlignmentType, TableCell,
    TableRow, WidthType,
};
use std::io::Cursor;

/// docx sizes are half-points.
fn half(pt: f32) -> usize {
    (pt * 2.0).round() as usize
}

fn fonted_run(text: &str) -> Run {
    Run::new()
        .add_text(text)
        .fonts(RunFonts::new().ascii(STYLE.font.family))
}

fn body_run(text: &str) -> Run {
    fonted_run(text).size(half(STYLE.body.size))
}

fn body_para(text: &str) -> Paragraph {
    Paragraph::new()
        .add_run(body_run(text))
        .line_spacing(docx_rs::LineSpacing::new().after(STYLE.spacing.body_after_twips as i32))
}

fn h2_para(text: &str) -> Paragraph {
    // No paragraph-border API in docx-rs 0.4's public surface (unlike the
    // JS `docx` library's `border: { bottom: ... }`) -- the acceptance bar
    // for this row is text-content parity, not visual parity, so the H2
    // underline is dropped rather than guessed at.
    Paragraph::new()
        .add_run(
            fonted_run(text)
                .size(half(STYLE.h2.size))
                .bold()
                .color(STYLE.h2.color),
        )
        .line_spacing(
            docx_rs::LineSpacing::new()
                .before(STYLE.spacing.h2_before_twips as i32)
                .after(STYLE.spacing.h2_after_twips as i32),
        )
}

fn meta_para(text: &str) -> Paragraph {
    Paragraph::new()
        .add_run(fonted_run(text).size(half(STYLE.meta.size)).color(STYLE.meta.color))
        .line_spacing(docx_rs::LineSpacing::new().after(STYLE.spacing.body_after_twips as i32))
}

/// Splits "lead-in **bold** phrase" into separate runs so a bullet can mix
/// bold and plain text in one paragraph -- same convention render_html.rs's
/// `inline_markup` establishes for the same source text.
fn bold_segments(text: &str) -> Vec<(String, bool)> {
    let mut out = Vec::new();
    let mut rest = text;
    loop {
        match rest.find("**") {
            None => {
                if !rest.is_empty() {
                    out.push((rest.to_string(), false));
                }
                break;
            }
            Some(start) => {
                let after = &rest[start + 2..];
                match after.find("**") {
                    None => {
                        out.push((rest.to_string(), false));
                        break;
                    }
                    Some(end) => {
                        if start > 0 {
                            out.push((rest[..start].to_string(), false));
                        }
                        out.push((after[..end].to_string(), true));
                        rest = &after[end + 2..];
                    }
                }
            }
        }
    }
    out
}

fn bullet_para(text: &str) -> Paragraph {
    // A real Word numbered-bullet list needs an AbstractNumbering
    // registered on the Docx itself, not just a per-paragraph reference --
    // out of scope for text-content parity (this row's acceptance bar).
    // A plain bullet-glyph prefix matches render_markdown.rs's "- " and
    // render_pdf.rs's "•  " prefixes: same content, same convention.
    let mut p = Paragraph::new().add_run(fonted_run("\u{2022}  ").size(half(STYLE.body.size)));
    for (seg, bold) in bold_segments(text) {
        let mut run = fonted_run(&seg).size(half(STYLE.body.size));
        if bold {
            run = run.bold();
        }
        p = p.add_run(run);
    }
    p.line_spacing(docx_rs::LineSpacing::new().after(STYLE.spacing.body_after_twips as i32))
}

fn table_cell_para(text: &str) -> Paragraph {
    Paragraph::new().add_run(body_run(text))
}

fn render_node_to_paras(node: &Node) -> Vec<Paragraph> {
    match node {
        Node::Heading { text, .. } => vec![h2_para(text)],
        Node::Paragraph { text } => vec![body_para(text)],
        Node::Bullets { items, .. } => items.iter().map(|i| bullet_para(i)).collect(),
        Node::CheckedBullets { items } => items
            .iter()
            .map(|i| {
                let text = if i.reason.is_empty() {
                    i.text.clone()
                } else {
                    format!("{} ({})", i.text, i.reason)
                };
                bullet_para(&text)
            })
            .collect(),
        Node::KvList { items } => items
            .iter()
            .map(|i| {
                Paragraph::new()
                    .add_run(fonted_run(&format!("{}  ", i.label)).size(half(STYLE.body.size)).bold())
                    .add_run(body_run(&i.value))
                    .line_spacing(docx_rs::LineSpacing::new().after(STYLE.spacing.body_after_twips as i32))
            })
            .collect(),
        Node::TruthCheck { sections_used, must_not_say } => {
            let mut paras = vec![body_para(&format!("Sections used: {}", sections_used.join(", ")))];
            if !must_not_say.is_empty() {
                paras.push(body_para("What we must not say:"));
                for m in must_not_say {
                    let text = if m.reason.is_empty() {
                        m.claim.clone()
                    } else {
                        format!("{} ({})", m.claim, m.reason)
                    };
                    paras.push(bullet_para(&text));
                }
            }
            paras
        }
        Node::Table { .. } => vec![],
    }
}

fn build_table(header: &[String], rows: &[Vec<String>]) -> Table {
    let mut all_rows = Vec::with_capacity(rows.len() + 1);
    all_rows.push(header.to_vec());
    all_rows.extend(rows.iter().cloned());
    let table_rows: Vec<TableRow> = all_rows
        .into_iter()
        .map(|r| {
            let cells: Vec<TableCell> = r
                .iter()
                .map(|c| TableCell::new().add_paragraph(table_cell_para(c)))
                .collect();
            TableRow::new(cells)
        })
        .collect();
    Table::new(table_rows)
        .align(TableAlignmentType::Center)
        .width(100 * 50, WidthType::Pct) // 100% == 5000 fiftieths-of-a-percent
}

/// `DocumentTree` -> a .docx `Vec<u8>` (via docx-rs's in-memory `build().pack()`).
/// `Box<dyn Error>` rather than naming docx-rs's zip-crate error type directly
/// -- that would require adding `zip` itself as a direct dependency just to
/// spell one return type.
pub fn render_docx(doc: &DocumentTree) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut docx = Docx::new().add_paragraph(
        Paragraph::new()
            .add_run(fonted_run(&doc.headline).size(half(STYLE.h1.size)).bold().color(STYLE.h1.color))
            .line_spacing(docx_rs::LineSpacing::new().after(20))
            .align(AlignmentType::Left),
    );

    if let Some(meta) = &doc.meta_line {
        docx = docx.add_paragraph(meta_para(meta));
    }

    for node in &doc.sections {
        match node {
            Node::Table { header, rows } => {
                docx = docx.add_table(build_table(header, rows));
            }
            other => {
                for p in render_node_to_paras(other) {
                    docx = docx.add_paragraph(p);
                }
            }
        }
    }

    if let Some(footer) = &doc.footer_note {
        docx = docx.add_paragraph(meta_para(footer));
    }

    docx = docx.page_size(STYLE.page.width_twips as u32, STYLE.page.height_twips as u32).page_margin(
        docx_rs::PageMargin::new()
            .top(STYLE.page.margin_top_twips as i32)
            .bottom(STYLE.page.margin_bottom_twips as i32)
            .left(STYLE.page.margin_left_twips as i32)
            .right(STYLE.page.margin_right_twips as i32)
            .header(STYLE.page.header_twips as i32)
            .footer(STYLE.page.footer_twips as i32),
    );

    let mut cursor = Cursor::new(Vec::new());
    docx.build().pack(&mut cursor)?;
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate::node_tree::*;

    #[test]
    fn renders_a_minimal_document_without_error() {
        let doc = document(DocumentSpec {
            headline: "MEMO".into(),
            meta_line: Some("subtitle".into()),
            sections: vec![paragraph("Hello."), table(vec!["A".into()], vec![vec!["1".into()]])],
            footer_note: Some("footer".into()),
        });
        let bytes = render_docx(&doc).unwrap();
        // A .docx is a zip archive -- assert the local-file-header magic bytes
        // rather than parsing the archive, which is enough to prove
        // `Packer`/`build().pack()` actually produced a real zip, not an
        // error swallowed into an empty buffer.
        assert_eq!(&bytes[0..2], b"PK");
        assert!(bytes.len() > 100);
    }

    #[test]
    fn bold_segments_splits_lead_in_bold_and_trailing_text() {
        let segs = bold_segments("lead-in **bold** trailing");
        assert_eq!(
            segs,
            vec![
                ("lead-in ".to_string(), false),
                ("bold".to_string(), true),
                (" trailing".to_string(), false),
            ]
        );
    }
}
