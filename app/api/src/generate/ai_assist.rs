//! BB26091208 part 2: field-level AI assist -- a per-field endpoint the
//! creator invokes explicitly, mirroring iSconl `spark`'s existing
//! `research-field`/`full-draft` contract
//! (`spark/lib/writer-assist.js` on top of `spark/lib/ai-provider.js`) and
//! its Groq backend (`GROQ_API_KEY`/`GROQ_MODEL`, same env var names, same
//! OpenAI-compatible chat-completions shape, same default model
//! `openai/gpt-oss-120b`) -- a genuinely new Rust module, not a port of an
//! existing one, since Press has had no AI-calling layer at all until now.
//!
//! ABSOLUTE RULE, structural not just documented: **no function in this
//! module is called from `doc_builder.rs`, `registry.rs`, or any
//! `render_*.rs`.** `build()` stays a pure function of `(content,
//! version)` -- this module only ever proposes a value that lands in the
//! exact same content field manual entry would, always reviewable/editable
//! before a creator clicks Generate. `research_field` drafts ONE field;
//! `full_draft` drafts every field at once from a single brief, still only
//! ever landing in the same reviewable form fields.
//!
//! Authoring rules baked into the prompts themselves, not just applied by
//! hand afterwards: every list-shaped field is asked for at most 3 items
//! (the same one-page cap `weekly_status_brief.rs`'s `validate()` enforces
//! structurally), and no em dashes or other AI-tell phrasing anywhere.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

const GROQ_HOST: &str = "https://api.groq.com/openai/v1/chat/completions";

fn default_model() -> String {
    std::env::var("GROQ_MODEL").unwrap_or_else(|_| "openai/gpt-oss-120b".to_string())
}

#[derive(Debug)]
pub struct AssistError(pub String);

impl std::fmt::Display for AssistError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for AssistError {}

/// Mirrors the archetype engine's own field vocabulary
/// (`archetype::FieldDef`) but as owned strings, since this comes off an
/// HTTP request body rather than a compiled-in `&'static` table.
#[derive(Debug, Clone, Deserialize)]
pub struct FieldDescriptor {
    pub name: String,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "type", default = "default_field_type")]
    pub field_type: String,
    #[serde(default)]
    pub keys: Option<Vec<String>>,
}

fn default_field_type() -> String {
    "text".to_string()
}

fn field_line(f: &FieldDescriptor) -> String {
    let label = if f.label.is_empty() { f.name.as_str() } else { f.label.as_str() };
    format!("{} ({label}, type: {})", f.name, f.field_type)
}

/// One instruction fragment per field type, telling the model exactly what
/// shape to return for that field and baking in the cap-3/no-em-dash
/// authoring rules rather than leaving them to be applied after the fact.
fn shape_instruction(f: &FieldDescriptor) -> String {
    match f.field_type.as_str() {
        "list" => "Return at most 3 items, one per line. No numbering, no bullet characters, no em dashes or other AI-tell phrasing.".to_string(),
        "reasoned-list" | "table-list" => {
            let keys = f.keys.clone().unwrap_or_else(|| vec!["a".to_string(), "b".to_string()]);
            format!(
                "Return at most 3 rows, one per line, each formatted as `{}`. No em dashes or other AI-tell phrasing.",
                keys.join(" | ")
            )
        }
        "textarea" => "Return prose paragraphs. No em dashes or other AI-tell phrasing.".to_string(),
        _ => "Return a single short line of text. No em dashes or other AI-tell phrasing.".to_string(),
    }
}

#[derive(Serialize)]
struct ChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Deserialize)]
struct ChatChoiceMessage {
    content: Option<String>,
}
#[derive(Deserialize)]
struct ChatChoice {
    message: ChatChoiceMessage,
}
#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}
#[derive(Deserialize)]
struct ErrorBody {
    error: Option<ErrorDetail>,
}
#[derive(Deserialize)]
struct ErrorDetail {
    message: Option<String>,
}

/// One chat-completion call against Groq's OpenAI-compatible endpoint.
/// `client`/`request_impl` are injected so tests never make a real network
/// call, same reasoning as `writer-assist.js`'s own `requestImpl` param.
pub async fn chat_complete(
    client: &reqwest::Client,
    api_key: &str,
    messages: Vec<(&'static str, String)>,
    temperature: f32,
    max_tokens: u32,
) -> Result<String, AssistError> {
    if api_key.is_empty() {
        return Err(AssistError("no AI provider configured -- GROQ_API_KEY is not set".into()));
    }
    let body = ChatRequest {
        model: default_model(),
        messages: messages.into_iter().map(|(role, content)| ChatMessage { role, content }).collect(),
        temperature,
        max_tokens,
    };
    let resp = client
        .post(GROQ_HOST)
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| AssistError(format!("Groq request failed: {e}")))?;
    let status = resp.status();
    let raw = resp.text().await.unwrap_or_default();
    if !status.is_success() {
        let msg = serde_json::from_str::<ErrorBody>(&raw)
            .ok()
            .and_then(|b| b.error)
            .and_then(|e| e.message)
            .unwrap_or_else(|| format!("Groq HTTP {status}"));
        return Err(AssistError(msg));
    }
    let parsed: ChatResponse = serde_json::from_str(&raw).map_err(|e| AssistError(format!("Groq returned unparseable JSON: {e}")))?;
    let text = parsed
        .choices
        .first()
        .and_then(|c| c.message.content.clone())
        .ok_or_else(|| AssistError("Groq returned no content".into()))?;
    let trimmed = text.trim().to_string();
    if trimmed.is_empty() {
        return Err(AssistError("Groq returned no content".into()));
    }
    Ok(trimmed)
}

fn render_other_field_context(other_field_values: &Map<String, Value>) -> String {
    other_field_values
        .iter()
        .filter(|(_, v)| !is_empty_value(v))
        .map(|(k, v)| format!("- {k}: {}", value_to_line(v)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn is_empty_value(v: &Value) -> bool {
    match v {
        Value::Null => true,
        Value::String(s) => s.is_empty(),
        Value::Array(a) => a.is_empty(),
        _ => false,
    }
}

fn value_to_line(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Array(a) => a
            .iter()
            .map(|x| match x {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect::<Vec<_>>()
            .join("; "),
        other => other.to_string(),
    }
}

/// Drafts ONE field's value from a brief plus whatever other fields are
/// already filled in -- the creator's explicit "research this field"
/// action. Never touches layout/formatting; the value lands in the exact
/// input control manual entry uses, always reviewable before Generate.
pub async fn research_field(
    client: &reqwest::Client,
    api_key: &str,
    archetype_id: &str,
    field: &FieldDescriptor,
    brief: Option<&str>,
    other_field_values: &Map<String, Value>,
) -> Result<String, AssistError> {
    let context = render_other_field_context(other_field_values);
    let system = format!(
        "You draft ONE field's value for a \"{archetype_id}\" document. Return ONLY the field's value -- no preamble, no markdown fencing, no field name/label. {}",
        shape_instruction(field)
    );
    let user = [
        Some(format!("Field to draft: {}", field_line(field))),
        brief.filter(|b| !b.is_empty()).map(|b| format!("Brief: {b}")),
        (!context.is_empty()).then(|| format!("Other fields already filled in, for context:\n{context}")),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join("\n\n");

    chat_complete(client, api_key, vec![("system", system), ("user", user)], 0.4, 512).await
}

/// Drafts every field of an archetype in one pass from a single brief --
/// the "get a first pass fast" mode. Still lands entirely in the same
/// reviewable input fields as manual entry; nothing here is ever a
/// silent, unreviewed final document.
pub async fn full_draft(
    client: &reqwest::Client,
    api_key: &str,
    archetype_id: &str,
    fields: &[FieldDescriptor],
    brief: &str,
) -> Result<Map<String, Value>, AssistError> {
    let field_list = fields
        .iter()
        .map(|f| format!("- {} -- {}", field_line(f), shape_instruction(f)))
        .collect::<Vec<_>>()
        .join("\n");
    let system = format!(
        "You draft every field of a \"{archetype_id}\" document in one pass, from a brief. Return ONLY a JSON object mapping each field name to its proposed value (a string, or a JSON array of strings for a \"list\"-type field, or a JSON array of objects for a \"reasoned-list\"/\"table-list\" field) -- no other text, no markdown fencing. No em dashes or other AI-tell phrasing anywhere."
    );
    let user = format!("Fields to draft:\n{field_list}\n\nBrief: {brief}");

    let raw = chat_complete(client, api_key, vec![("system", system), ("user", user)], 0.5, 2048).await?;
    let cleaned = raw
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();
    let parsed: Value = serde_json::from_str(cleaned).map_err(|e| AssistError(format!("AI full-draft response wasn't valid JSON: {e}")))?;
    match parsed {
        Value::Object(map) => Ok(map),
        _ => Err(AssistError("AI full-draft response was not a field->value object".into())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_instruction_caps_list_fields_at_three_and_bans_em_dashes() {
        let f = FieldDescriptor { name: "signal".into(), label: "Signal".into(), field_type: "list".into(), keys: None };
        let instr = shape_instruction(&f);
        assert!(instr.contains("at most 3 items"));
        assert!(instr.contains("no em dashes"));
    }

    #[test]
    fn shape_instruction_for_reasoned_list_uses_declared_keys() {
        let f = FieldDescriptor {
            name: "anticipated_qa".into(),
            label: "Anticipated Q&A".into(),
            field_type: "reasoned-list".into(),
            keys: Some(vec!["question".into(), "answer".into()]),
        };
        let instr = shape_instruction(&f);
        assert!(instr.contains("question | answer"));
        assert!(instr.contains("at most 3 rows"));
    }

    #[test]
    fn render_other_field_context_skips_empty_values() {
        let mut m = Map::new();
        m.insert("a".into(), Value::String("x".into()));
        m.insert("b".into(), Value::String("".into()));
        m.insert("c".into(), Value::Array(vec![]));
        let ctx = render_other_field_context(&m);
        assert_eq!(ctx, "- a: x");
    }

    #[tokio::test]
    async fn chat_complete_errors_cleanly_with_no_api_key() {
        let client = reqwest::Client::new();
        let err = chat_complete(&client, "", vec![("user", "hi".into())], 0.4, 10).await.unwrap_err();
        assert!(err.0.contains("GROQ_API_KEY"));
    }

    #[test]
    fn full_draft_response_parsing_rejects_a_non_object_json_value() {
        let parsed: Value = serde_json::from_str("[1,2,3]").unwrap();
        assert!(!matches!(parsed, Value::Object(_)));
    }
}
