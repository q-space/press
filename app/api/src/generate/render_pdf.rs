//! PDF renderer -- built directly from the node tree, never by converting
//! the generated `.docx` (canon §7). Ported from
//! `scope/lib/generate/render-pdf.js`, which builds on `pdfkit`'s built-in
//! (non-embedded) Helvetica. `genpdf` has no equivalent: even its
//! `fonts::Builtin` name is only a metrics label passed alongside a real
//! font file -- `FontData::load`/`from_files` always reads actual TTF bytes
//! from disk, with no fallback that skips embedding (verified against the
//! crate's own source before writing this, not assumed). `printpdf`
//! (current stable, a full API rewrite from the version this decision might
//! otherwise have assumed) turns out to have dropped its own
//! builtin-font example in the same way. **Decision: vendor Liberation
//! Sans (OFL-licensed, metrically compatible with Helvetica/Arial) as 4
//! `include_bytes!` faces under `assets/fonts/` and embed them at compile
//! time** -- no runtime file I/O, no missing-file failure mode, and the
//! visual result matches pdfkit's own Helvetica output closely enough that
//! this is a rendering-parity win, not just a workaround. See
//! `work/_arc/qspace-press/canon-canvas/SESSION-RECORD.md` for the full
//! reasoning and the rejected alternatives.

use super::node_tree::{DocumentTree, Node};
use super::style::STYLE;
use genpdf::elements::{Paragraph, TableLayout};
use genpdf::fonts::{FontData, FontFamily};
use genpdf::style::{Color, Style};
use genpdf::{Alignment, Document, Element as _, Margins, Mm, SimplePageDecorator, Size};

fn twips_to_mm(twips: u32) -> f64 {
    twips as f64 / 1440.0 * 25.4
}

/// `STYLE`'s colors are bare hex strings ("1F3864") -- same convention
/// render_html.rs's CSS interpolation reads them with.
fn hex_color(hex: &str) -> Color {
    let bytes = u32::from_str_radix(hex, 16).unwrap_or(0);
    Color::Rgb(
        ((bytes >> 16) & 0xff) as u8,
        ((bytes >> 8) & 0xff) as u8,
        (bytes & 0xff) as u8,
    )
}

fn font_family() -> FontFamily<FontData> {
    // Embedded at compile time -- see the module doc comment for why no
    // disk-based font loading is used.
    let regular = include_bytes!("../../assets/fonts/LiberationSans-Regular.ttf").to_vec();
    let bold = include_bytes!("../../assets/fonts/LiberationSans-Bold.ttf").to_vec();
    let italic = include_bytes!("../../assets/fonts/LiberationSans-Italic.ttf").to_vec();
    let bold_italic = include_bytes!("../../assets/fonts/LiberationSans-BoldItalic.ttf").to_vec();
    FontFamily {
        regular: FontData::new(regular, None).expect("embedded LiberationSans-Regular.ttf is malformed"),
        bold: FontData::new(bold, None).expect("embedded LiberationSans-Bold.ttf is malformed"),
        italic: FontData::new(italic, None).expect("embedded LiberationSans-Italic.ttf is malformed"),
        bold_italic: FontData::new(bold_italic, None)
            .expect("embedded LiberationSans-BoldItalic.ttf is malformed"),
    }
}

// `Paragraph::styled()` (from the `Element` trait) returns a wrapping
// `StyledElement<Paragraph>`, not a `Paragraph` -- these helpers stay
// `Paragraph`-returning (so `push_node` and the table-cell code below can
// keep treating every node the same way) by building through
// `styled_string` instead, same as `kv_para` already does.
fn heading_para(text: &str) -> Paragraph {
    Paragraph::default().styled_string(
        text.to_string(),
        Style::new().bold().with_font_size((STYLE.h2.size + 3.0) as u8).with_color(hex_color(STYLE.h2.color)),
    )
}

fn body_para(text: &str) -> Paragraph {
    Paragraph::default().styled_string(text.to_string(), Style::new().with_font_size((STYLE.body.size + 2.0) as u8))
}

fn bullet_line(text: &str) -> Paragraph {
    body_para(&format!("\u{2022}  {text}"))
}

fn kv_para(label: &str, value: &str) -> Paragraph {
    Paragraph::default()
        .styled_string(format!("{label}  "), Style::new().bold().with_font_size((STYLE.body.size + 2.0) as u8))
        .styled_string(value.to_string(), Style::new().with_font_size((STYLE.body.size + 2.0) as u8))
}

fn push_node(doc: &mut Document, node: &Node) {
    match node {
        Node::Heading { text, .. } => doc.push(heading_para(text)),
        Node::Paragraph { text } => doc.push(body_para(text)),
        Node::Bullets { items, .. } => {
            for i in items {
                doc.push(bullet_line(i));
            }
        }
        Node::CheckedBullets { items } => {
            for i in items {
                let text = if i.reason.is_empty() { i.text.clone() } else { format!("{} ({})", i.text, i.reason) };
                doc.push(bullet_line(&text));
            }
        }
        Node::KvList { items } => {
            for i in items {
                doc.push(kv_para(&i.label, &i.value));
            }
        }
        Node::TruthCheck { sections_used, must_not_say } => {
            doc.push(body_para(&format!("Sections used: {}", sections_used.join(", "))));
            if !must_not_say.is_empty() {
                doc.push(body_para("What we must not say:"));
                for m in must_not_say {
                    let text = if m.reason.is_empty() { m.claim.clone() } else { format!("{} ({})", m.claim, m.reason) };
                    doc.push(bullet_line(&text));
                }
            }
        }
        Node::Table { header, rows } => {
            let mut table = TableLayout::new(vec![1; header.len().max(1)]);
            let mut header_row = table.row();
            for h in header {
                header_row = header_row.element(body_para(h));
            }
            header_row.push().expect("header row width matches column count");
            for r in rows {
                let mut row = table.row();
                for c in r {
                    row = row.element(body_para(c));
                }
                row.push().expect("row width matches column count");
            }
            doc.push(table);
        }
    }
}

/// `DocumentTree` -> a `.pdf` `Vec<u8>`.
pub fn render_pdf(doc_tree: &DocumentTree) -> Result<Vec<u8>, genpdf::error::Error> {
    let mut doc = Document::new(font_family());
    doc.set_title(doc_tree.headline.clone());
    doc.set_paper_size(Size::new(
        Mm(twips_to_mm(STYLE.page.width_twips)),
        Mm(twips_to_mm(STYLE.page.height_twips)),
    ));

    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(Margins::trbl(
        Mm(twips_to_mm(STYLE.page.margin_top_twips)),
        Mm(twips_to_mm(STYLE.page.margin_right_twips)),
        Mm(twips_to_mm(STYLE.page.margin_bottom_twips)),
        Mm(twips_to_mm(STYLE.page.margin_left_twips)),
    ));
    doc.set_page_decorator(decorator);

    doc.push(
        Paragraph::new(doc_tree.headline.clone())
            .aligned(Alignment::Left)
            .styled(Style::new().bold().with_font_size((STYLE.h1.size + 5.0) as u8).with_color(hex_color(STYLE.h1.color))),
    );
    if let Some(meta) = &doc_tree.meta_line {
        doc.push(Paragraph::new(meta.clone()).styled(Style::new().with_font_size((STYLE.meta.size + 2.0) as u8).with_color(hex_color(STYLE.meta.color))));
    }
    for node in &doc_tree.sections {
        push_node(&mut doc, node);
    }
    if let Some(footer) = &doc_tree.footer_note {
        doc.push(Paragraph::new(footer.clone()).styled(Style::new().with_font_size((STYLE.meta.size + 2.0) as u8).with_color(hex_color(STYLE.meta.color))));
    }

    let mut buf = Vec::new();
    doc.render(&mut buf)?;
    Ok(buf)
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
        let bytes = render_pdf(&doc).unwrap();
        // A .pdf starts with "%PDF-" -- assert the magic header rather than
        // parsing the file, which is enough to prove a real PDF stream came
        // out rather than an error being swallowed into an empty buffer.
        assert_eq!(&bytes[0..5], b"%PDF-");
    }

    #[test]
    fn hex_color_parses_the_house_style_h1_color() {
        assert_eq!(hex_color("1F3864"), Color::Rgb(0x1F, 0x38, 0x64));
    }
}
