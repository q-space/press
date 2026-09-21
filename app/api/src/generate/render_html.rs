//! HTML renderer -- fourth, independent output alongside docx/markdown/pdf,
//! built directly from the same node tree + `style.rs` spec per canon §7's
//! "one content tree, N outputs, zero AI calls" pattern. Ported from
//! `scope/lib/generate/render-html.js` (BA26090601 in iSconl).
//!
//! Structural feature unique to this renderer: each bullet is individually
//! expandable via a plain `<details>/<summary>` element (no JS framework),
//! reading `Node::Bullets`'s optional `details` field that no other
//! renderer looks at.
//!
//! Output is one self-contained HTML string -- inline `<style>` only, zero
//! external asset/network dependency.

use super::node_tree::{DocumentTree, Node};
use super::style::STYLE;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Same lead-in-**bold** convention render_docx.rs's bold-segment parser
/// establishes for bullet text -- reused here so `**bold**` phrases render
/// consistently across formats instead of showing literal asterisks.
fn inline_markup(text: &str) -> String {
    let escaped = esc(text);
    let mut out = String::with_capacity(escaped.len());
    let mut rest = escaped.as_str();
    loop {
        match rest.find("**") {
            None => {
                out.push_str(rest);
                break;
            }
            Some(start) => {
                let after = &rest[start + 2..];
                match after.find("**") {
                    None => {
                        out.push_str(rest);
                        break;
                    }
                    Some(end) => {
                        out.push_str(&rest[..start]);
                        out.push_str("<strong>");
                        out.push_str(&after[..end]);
                        out.push_str("</strong>");
                        rest = &after[end + 2..];
                    }
                }
            }
        }
    }
    out
}

fn render_bullet_items(items: &[String], details: &Option<Vec<Option<String>>>) -> String {
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let detail = details.as_ref().and_then(|d| d.get(i)).and_then(|d| d.as_ref());
            match detail {
                None => format!("<li>{}</li>", inline_markup(item)),
                Some(detail) => format!(
                    "<li><details><summary>{}</summary><div class=\"bullet-detail\">{}</div></details></li>",
                    inline_markup(item),
                    inline_markup(detail)
                ),
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_node(node: &Node) -> String {
    match node {
        Node::Heading { text, .. } => format!("<h2>{}</h2>", esc(text)),
        Node::Paragraph { text } => format!("<p>{}</p>", inline_markup(text)),
        Node::Bullets { items, details } => {
            format!("<ul class=\"bullets\">\n{}\n</ul>", render_bullet_items(items, details))
        }
        Node::CheckedBullets { items } => {
            let lis: Vec<String> = items
                .iter()
                .map(|i| {
                    if i.reason.is_empty() {
                        format!("<li>{}</li>", inline_markup(&i.text))
                    } else {
                        format!(
                            "<li>{} <span class=\"reason\">({})</span></li>",
                            inline_markup(&i.text),
                            esc(&i.reason)
                        )
                    }
                })
                .collect();
            format!("<ul class=\"bullets checked\">\n{}\n</ul>", lis.join("\n"))
        }
        Node::KvList { items } => {
            let rows: Vec<String> = items
                .iter()
                .map(|i| {
                    format!(
                        "<div class=\"kv-row\"><dt>{}</dt><dd>{}</dd></div>",
                        esc(&i.label),
                        inline_markup(&i.value)
                    )
                })
                .collect();
            format!("<dl class=\"kv\">\n{}\n</dl>", rows.join("\n"))
        }
        Node::TruthCheck { sections_used, must_not_say } => {
            let mut parts = vec![format!("<p>Sections used: {}</p>", esc(&sections_used.join(", ")))];
            if !must_not_say.is_empty() {
                parts.push("<p><strong>What we must not say:</strong></p>".to_string());
                let lis: Vec<String> = must_not_say
                    .iter()
                    .map(|m| {
                        if m.reason.is_empty() {
                            format!("<li>{}</li>", inline_markup(&m.claim))
                        } else {
                            format!(
                                "<li>{} <span class=\"reason\">({})</span></li>",
                                inline_markup(&m.claim),
                                esc(&m.reason)
                            )
                        }
                    })
                    .collect();
                parts.push(format!("<ul class=\"bullets\">\n{}\n</ul>", lis.join("\n")));
            }
            parts.join("\n")
        }
        Node::Table { header, rows } => {
            let thead: String = header.iter().map(|h| format!("<th>{}</th>", esc(h))).collect();
            let tbody: Vec<String> = rows
                .iter()
                .map(|r| {
                    let cells: String = r.iter().map(|c| format!("<td>{}</td>", esc(c))).collect();
                    format!("<tr>{cells}</tr>")
                })
                .collect();
            format!(
                "<table>\n<thead><tr>{thead}</tr></thead>\n<tbody>{}</tbody>\n</table>",
                tbody.join("\n")
            )
        }
    }
}

fn pt2px(pt: f32) -> i32 {
    (pt * 4.0 / 3.0).round() as i32
}
fn margin_px(twips: u32) -> i32 {
    ((twips as f32) / 1440.0 * 96.0).round() as i32
}
/// The same page margin expressed for `@page`, which is the only thing
/// that sets the printed margin -- see the `@page`/`@media print` block in
/// `css()` for why the on-screen padding cannot be it.
fn margin_mm(twips: u32) -> String {
    format!("{:.1}", (twips as f32) / 1440.0 * 25.4)
}

fn css() -> String {
    format!(
        r#":root {{ color-scheme: light; }}
body {{
  margin: 0; background: #f4f6fa;
  font-family: '{font}', Calibri, 'Segoe UI', Arial, sans-serif;
  color: #000; font-size: {body_size}px; line-height: {body_line};
}}
.page {{
  max-width: 900px; margin: 24px auto; background: #fff;
  padding: {mtop}px {mright}px {mbottom}px {mleft}px;
  box-shadow: 0 1px 4px rgba(0,0,0,0.12);
}}
h1 {{
  font-size: {h1_size}px; color: #{h1_color}; font-weight: 700;
  margin: 0 0 4px 0;
}}
.meta-line {{
  font-size: {meta_size}px; color: #{meta_color}; margin: 0 0 18px 0;
}}
h2 {{
  font-size: {h2_size}px; color: #{h2_color}; font-weight: 700;
  border-bottom: 1px solid #{h2_border};
  padding-bottom: 4px; margin: {h2_before}px 0 {h2_after}px 0;
}}
p {{ margin: 0 0 {body_after}px 0; }}
ul.bullets {{ margin: 0 0 10px 0; padding-left: 22px; }}
ul.bullets li {{ margin: 0 0 6px 0; }}
ul.bullets .reason {{ color: #{meta_color}; font-size: {meta_size_1}px; }}
dl.kv {{ margin: 0 0 10px 0; }}
.kv-row {{ margin: 0 0 6px 0; }}
.kv-row dt {{ display: inline; font-weight: 700; margin: 0; }}
.kv-row dt::after {{ content: '  '; }}
.kv-row dd {{ display: inline; margin: 0; }}
table {{ border-collapse: collapse; width: 100%; margin: 0 0 12px 0; }}
th, td {{ border: 1px solid #ccc; padding: 4px 8px; text-align: left; font-size: {body_px}px; }}
th {{ background: #f0f2f8; }}
.footer-note {{
  margin-top: 24px; padding-top: 10px; border-top: 1px solid #ddd;
  font-size: {meta_size}px; color: #{meta_color};
}}
details {{ margin: 0 0 6px 0; }}
details > summary {{ cursor: pointer; list-style: none; }}
details > summary::-webkit-details-marker {{ display: none; }}
details > summary::before {{ content: '\25b8  '; color: #{h2_color}; font-size: 0.85em; }}
details[open] > summary::before {{ content: '\25be  '; }}
.bullet-detail {{
  margin: 4px 0 0 18px; padding: 6px 10px;
  background: #f7f8fc; border-left: 2px solid #{h2_border};
  font-size: {body_px}px; color: #333;
}}

/* Print/PDF. The printed margin comes from @page and never from .page's
   own padding, so an overflow page gets the same margin as the first --
   padding only indents the first page's content and leaves page 2 flush
   to the sheet edge.

   .page itself keeps no fixed height, no aspect-ratio and no overflow,
   on screen or in print. Both of those tricks silently LOSE content
   rather than misplacing it: a print engine does not paginate a scrolling
   container, it clips it to the box height, so everything past the fold
   vanishes from the PDF instead of flowing to page 2. Natural content
   flow means an overflow on screen is an honest signal that the PDF
   spills to a second page too, which is what makes the Creator Studio
   preview (BB26091204) the output rather than an approximation of it. */
@page {{ size: A4; margin: {mm_top}mm {mm_right}mm {mm_bottom}mm {mm_left}mm; }}
@media print {{
  body {{ background: #fff; }}
  .page {{ max-width: none; margin: 0; padding: 0; box-shadow: none; }}
  tr, li, .kv-row, details {{ break-inside: avoid; }}
  h1, h2 {{ break-after: avoid; }}
}}"#,
        font = STYLE.font.family,
        body_size = pt2px(STYLE.body.size),
        body_line = STYLE.spacing.body_line,
        mtop = margin_px(STYLE.page.margin_top_twips),
        mright = margin_px(STYLE.page.margin_right_twips),
        mbottom = margin_px(STYLE.page.margin_bottom_twips),
        mleft = margin_px(STYLE.page.margin_left_twips),
        mm_top = margin_mm(STYLE.page.margin_top_twips),
        mm_right = margin_mm(STYLE.page.margin_right_twips),
        mm_bottom = margin_mm(STYLE.page.margin_bottom_twips),
        mm_left = margin_mm(STYLE.page.margin_left_twips),
        h1_size = pt2px(STYLE.h1.size + 5.0),
        h1_color = STYLE.h1.color,
        meta_size = pt2px(STYLE.meta.size + 2.0),
        meta_color = STYLE.meta.color,
        h2_size = pt2px(STYLE.h2.size + 3.0),
        h2_color = STYLE.h2.color,
        h2_border = STYLE.h2.border_color.unwrap_or(""),
        h2_before = margin_px(STYLE.spacing.h2_before_twips),
        h2_after = margin_px(STYLE.spacing.h2_after_twips),
        body_after = margin_px(STYLE.spacing.body_after_twips),
        meta_size_1 = pt2px(STYLE.meta.size + 1.0),
        body_px = pt2px(STYLE.body.size),
    )
}

/// `DocumentTree` -> a self-contained HTML string (no external asset/
/// network dependency -- inline `<style>` only).
pub fn render_html(doc: &DocumentTree) -> String {
    let mut body_parts = vec!["<div class=\"page\">".to_string(), format!("<h1>{}</h1>", esc(&doc.headline))];
    if let Some(meta) = &doc.meta_line {
        body_parts.push(format!("<p class=\"meta-line\">{}</p>", esc(meta)));
    }
    for node in &doc.sections {
        body_parts.push(render_node(node));
    }
    if let Some(footer) = &doc.footer_note {
        body_parts.push(format!("<p class=\"footer-note\">{}</p>", inline_markup(footer)));
    }
    body_parts.push("</div>".to_string());
    let body = body_parts.into_iter().filter(|p| !p.is_empty()).collect::<Vec<_>>().join("\n");

    format!(
        "<!doctype html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{}</title>\n<style>\n{}\n</style>\n</head>\n<body>\n{}\n</body>\n</html>\n",
        esc(&doc.headline),
        css(),
        body
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate::node_tree::*;

    #[test]
    fn escapes_the_headline_and_wraps_bold_markup() {
        let doc = document(DocumentSpec {
            headline: "<Title>".into(),
            meta_line: None,
            sections: vec![paragraph("say **hi** to <you>")],
            footer_note: None,
        });
        let html = render_html(&doc);
        assert!(html.contains("<title>&lt;Title&gt;</title>"));
        assert!(html.contains("<p>say <strong>hi</strong> to &lt;you&gt;</p>"));
    }

    #[test]
    fn expandable_bullet_only_when_a_detail_exists() {
        let node = bullets_with_details(vec!["a", "b"], vec![Some("more on a"), None]);
        let html = render_node(&node);
        assert!(html.contains("<details><summary>a</summary>"));
        assert!(html.contains("<li>b</li>"));
    }

    /// The print rules are what make the Creator Studio preview the same
    /// document as the export rather than a lookalike, so they are
    /// asserted rather than left to inspection. The negative assertion
    /// matters most: a fixed height or a scrolling container would clip
    /// the PDF instead of paginating it, and the loss is silent.
    #[test]
    fn print_css_sets_the_page_margin_and_never_paginates_a_clipped_box() {
        let sheet = css();
        assert!(sheet.contains("@page"));
        assert!(sheet.contains("size: A4"));
        assert!(sheet.contains("@media print"));
        assert!(sheet.contains(".page { max-width: none; margin: 0; padding: 0; box-shadow: none; }"));
        assert!(sheet.contains("break-inside: avoid"));
        assert!(!sheet.contains("aspect-ratio:"));
        assert!(!sheet.contains("overflow:"));
        assert!(!sheet.contains("overflow-y"));
        assert!(!sheet.contains("max-height:"));
    }

    #[test]
    fn page_margin_is_expressed_in_millimetres_for_at_page() {
        assert_eq!(margin_mm(1440), "25.4");
        assert_eq!(margin_mm(720), "12.7");
    }
}
