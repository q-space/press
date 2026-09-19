//! The neutral node-tree vocabulary every archetype builds from and every
//! renderer walks. Ported from iSconl `scope/lib/generate/node-tree.js`
//! (BB26091203) -- same shapes, `enum Node` standing in for JS's tagged
//! plain objects. `Serialize`/`Deserialize` are added even though nothing
//! wires them to an API yet (BB26091205 does that): a `Content`-adjacent
//! tree crossing the wire is the obvious next use, and deriving it now
//! costs nothing.
//!
//! See `scope/docs/document-generation-canon.md` §1/§3 for the design.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckedBulletItem {
    pub text: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KvItem {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MustNotSayItem {
    pub claim: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph {
        text: String,
    },
    Bullets {
        items: Vec<String>,
        /// Same-length, additive per-item longer text (JS: `node.details`,
        /// optional array with `null` where a bullet has none). Only the
        /// HTML renderer reads this (BA26090601) -- docx/md/pdf ignore it,
        /// same as upstream.
        #[serde(skip_serializing_if = "Option::is_none")]
        details: Option<Vec<Option<String>>>,
    },
    CheckedBullets {
        items: Vec<CheckedBulletItem>,
    },
    KvList {
        items: Vec<KvItem>,
    },
    TruthCheck {
        sections_used: Vec<String>,
        must_not_say: Vec<MustNotSayItem>,
    },
    Table {
        header: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

pub fn heading(level: u8, text: impl Into<String>) -> Node {
    Node::Heading { level, text: text.into() }
}

pub fn paragraph(text: impl Into<String>) -> Node {
    Node::Paragraph { text: text.into() }
}

/// A run of paragraphs under one section -- kept as separate nodes rather
/// than one joined string, mirroring node-tree.js's `paragraphs()`. Empty
/// strings are dropped, same as the JS `.filter(Boolean)`.
pub fn paragraphs<I, S>(texts: I) -> Vec<Node>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    texts
        .into_iter()
        .map(Into::into)
        .filter(|t: &String| !t.is_empty())
        .map(paragraph)
        .collect()
}

pub fn bullets<I, S>(items: I) -> Node
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    Node::Bullets {
        items: items.into_iter().map(Into::into).collect(),
        details: None,
    }
}

pub fn bullets_with_details<I, S, D>(items: I, details: Vec<Option<D>>) -> Node
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
    D: Into<String>,
{
    Node::Bullets {
        items: items.into_iter().map(Into::into).collect(),
        details: Some(details.into_iter().map(|d| d.map(Into::into)).collect()),
    }
}

pub fn checked_bullets(items: Vec<(String, String)>) -> Node {
    Node::CheckedBullets {
        items: items
            .into_iter()
            .map(|(text, reason)| CheckedBulletItem { text, reason })
            .collect(),
    }
}

pub fn kv_list<L, V>(pairs: Vec<(L, V)>) -> Node
where
    L: Into<String>,
    V: Into<String>,
{
    Node::KvList {
        items: pairs
            .into_iter()
            .map(|(label, value)| KvItem { label: label.into(), value: value.into() })
            .collect(),
    }
}

pub fn truth_check(sections_used: Vec<String>, must_not_say: Vec<(String, String)>) -> Node {
    Node::TruthCheck {
        sections_used,
        must_not_say: must_not_say
            .into_iter()
            .map(|(claim, reason)| MustNotSayItem { claim, reason })
            .collect(),
    }
}

pub fn table(header: Vec<String>, rows: Vec<Vec<String>>) -> Node {
    Node::Table { header, rows }
}

/// A document is a header block (headline + optional meta line) plus an
/// ordered list of body section nodes plus an optional footer note -- the
/// shape every archetype's `build()` output conforms to (node-tree.js's
/// `document()`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentTree {
    pub headline: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta_line: Option<String>,
    pub sections: Vec<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_note: Option<String>,
}

pub struct DocumentSpec {
    pub headline: String,
    pub meta_line: Option<String>,
    pub sections: Vec<Node>,
    pub footer_note: Option<String>,
}

pub fn document(spec: DocumentSpec) -> DocumentTree {
    DocumentTree {
        headline: spec.headline,
        meta_line: spec.meta_line,
        sections: spec.sections,
        footer_note: spec.footer_note,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraphs_drops_empty_strings() {
        let out = paragraphs(vec!["a", "", "b"]);
        assert_eq!(out, vec![paragraph("a"), paragraph("b")]);
    }

    #[test]
    fn bullets_without_details_serializes_without_the_field() {
        let json = serde_json::to_value(bullets(vec!["a", "b"])).unwrap();
        assert_eq!(json["type"], "bullets");
        assert_eq!(json["items"], serde_json::json!(["a", "b"]));
        assert!(json.get("details").is_none());
    }
}
