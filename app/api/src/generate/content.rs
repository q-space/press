//! `Content` is the dynamically-shaped field map every archetype's
//! `validate()`/`build()` reads from -- JS content objects have no fixed
//! struct (each archetype declares its own `fields`), so `serde_json::Map`
//! is the direct Rust equivalent rather than inventing one struct per
//! archetype. This is also exactly the shape a JSON request body decodes
//! into, which is what `Content` will carry once BB26091205 wires an HTTP
//! endpoint in front of this.

use serde_json::Value;

pub type Content = serde_json::Map<String, Value>;

pub fn get_str<'a>(content: &'a Content, field: &str) -> Option<&'a str> {
    content.get(field).and_then(|v| v.as_str())
}

pub fn get_str_array(content: &Content, field: &str) -> Option<Vec<String>> {
    content.get(field).and_then(|v| v.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|x| x.as_str().map(|s| s.to_string()))
            .collect()
    })
}

/// Array of `{claim, reason}` / `{text, reason}`-shaped objects -- read
/// `key` and `reason` off each element, skipping any element missing `key`.
pub fn get_pair_array(content: &Content, field: &str, key: &str) -> Option<Vec<(String, String)>> {
    content.get(field).and_then(|v| v.as_array()).map(|arr| {
        arr.iter()
            .filter_map(|item| {
                let obj = item.as_object()?;
                let text = obj.get(key)?.as_str()?.to_string();
                let reason = obj
                    .get("reason")
                    .and_then(|r| r.as_str())
                    .unwrap_or("")
                    .to_string();
                Some((text, reason))
            })
            .collect()
    })
}

/// Mirrors the missing-field convention every archetype's JS `validate()`
/// repeats: absent, null, empty string, and empty array all count as
/// "missing" -- everything else (including `0`/`false`) counts as present.
pub fn is_missing(content: &Content, field: &str) -> bool {
    match content.get(field) {
        None => true,
        Some(Value::Null) => true,
        Some(Value::String(s)) => s.is_empty(),
        Some(Value::Array(a)) => a.is_empty(),
        Some(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn content(v: Value) -> Content {
        v.as_object().unwrap().clone()
    }

    #[test]
    fn is_missing_treats_empty_string_and_empty_array_as_missing() {
        let c = content(json!({"a": "", "b": [], "c": "x", "d": ["y"], "e": 0}));
        assert!(is_missing(&c, "a"));
        assert!(is_missing(&c, "b"));
        assert!(!is_missing(&c, "c"));
        assert!(!is_missing(&c, "d"));
        assert!(!is_missing(&c, "e"));
        assert!(is_missing(&c, "missing_key"));
    }
}
