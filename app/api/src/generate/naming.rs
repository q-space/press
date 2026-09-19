//! The general filename shape (canon §5): two identifying slots, version,
//! date, no free-text descriptor. Ported from `scope/lib/generate/naming.js`.
//! This module only knows the shape -- it never hardcodes what the two
//! slots mean, same as upstream; each archetype supplies its own
//! `filename_fields`.

use super::content::{get_str, Content};
use chrono::NaiveDate;

pub fn slugify(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_was_dash = true; // suppresses a leading dash
    for c in s.trim().to_lowercase().chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_was_dash = false;
        } else if !last_was_dash {
            out.push('-');
            last_was_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    out
}

#[derive(Debug)]
pub struct NamingError(pub String);

impl std::fmt::Display for NamingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for NamingError {}

pub struct FilenameOptions<'a> {
    pub version: &'a str,
    pub date: NaiveDate,
    pub ext: &'a str,
}

/// `filename_fields` = (primary field name, secondary field name), the
/// same pair an archetype module declares as `filenameFields` in the JS
/// registry. Errors the same way naming.js does: a specific, actionable
/// message rather than silently emitting a malformed filename.
pub fn filename(
    archetype_id: &str,
    filename_fields: (&str, &str),
    content: &Content,
    opts: &FilenameOptions,
) -> Result<String, NamingError> {
    let (primary, secondary) = filename_fields;
    let primary_val = slugify(get_str(content, primary).unwrap_or(""));
    let secondary_val = slugify(get_str(content, secondary).unwrap_or(""));
    if primary_val.is_empty() || secondary_val.is_empty() {
        return Err(NamingError(format!(
            "naming: content is missing \"{primary}\" or \"{secondary}\" (archetype \"{archetype_id}\"'s filename fields)"
        )));
    }
    let mut version_parts = opts.version.split('.');
    let major = version_parts.next().unwrap_or("0");
    let minor = version_parts.next().unwrap_or("0");
    let patch = version_parts.next().unwrap_or("0");
    let yyyymmdd = opts.date.format("%Y%m%d");
    Ok(format!(
        "{primary_val}_{secondary_val}_v{major}_{minor}_{patch}_{yyyymmdd}.{}",
        opts.ext
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn content(v: serde_json::Value) -> Content {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn slugify_matches_the_js_regex_behavior() {
        assert_eq!(slugify("Members / Member Services"), "members-member-services");
        assert_eq!(slugify("  Hello, World!  "), "hello-world");
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn filename_builds_the_canon_shape() {
        let c = content(json!({"re": "Q3 update", "date": "2026-09-14"}));
        let opts = FilenameOptions {
            version: "1.4.0",
            date: NaiveDate::from_ymd_opt(2026, 9, 14).unwrap(),
            ext: "docx",
        };
        let out = filename("single-page-memo", ("re", "date"), &c, &opts).unwrap();
        assert_eq!(out, "q3-update_2026-09-14_v1_4_0_20260914.docx");
    }

    #[test]
    fn filename_errors_when_a_slot_field_is_missing() {
        let c = content(json!({"re": "Q3 update"}));
        let opts = FilenameOptions { version: "1.0.0", date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(), ext: "md" };
        let err = filename("single-page-memo", ("re", "date"), &c, &opts).unwrap_err();
        assert!(err.0.contains("missing"));
    }
}
