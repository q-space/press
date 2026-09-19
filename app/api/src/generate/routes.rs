//! Minimal HTTP surface for this row -- just enough that part 2's
//! field-level AI assist is actually reachable and part 1's archetype is
//! end-to-end verifiable over HTTP, not the full `/archetypes/*` CRUD
//! surface BB26091205 (thin-client API, not yet started -- `main.rs` had
//! only `/health` before this row) will eventually own. Scoped
//! deliberately narrow:
//!   GET  /archetypes/:id/schema
//!   POST /archetypes/preview            (namespace, archetypeId, content) -> html
//!   POST /archetypes/generate           (namespace, archetypeId, content, format) -> file bytes
//!   POST /archetypes/:id/ai-assist/research-field
//!   POST /archetypes/:id/ai-assist/full-draft
//! A future BB26091205 session should read this file before adding the
//! rest, not duplicate it.

use crate::generate::archetype::FieldDef;
use crate::generate::ai_assist::{self, FieldDescriptor};
use crate::generate::content::Content;
use crate::generate::doc_registry::DocRegistry;
use crate::generate::{doc_builder, registry, render_docx, render_html, render_markdown, render_pdf};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use base64::Engine;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Clone)]
pub struct GenerateState {
    pub client: reqwest::Client,
    pub groq_api_key: String,
    pub doc_registry: Arc<DocRegistry>,
}

pub fn router(state: GenerateState) -> Router {
    Router::new()
        .route("/archetypes/:id/schema", get(get_schema))
        .route("/archetypes/preview", post(post_preview))
        .route("/archetypes/generate", post(post_generate))
        .route("/archetypes/:id/ai-assist/research-field", post(post_research_field))
        .route("/archetypes/:id/ai-assist/full-draft", post(post_full_draft))
        .with_state(state)
}

fn field_def_to_descriptor(f: &FieldDef) -> FieldDescriptor {
    FieldDescriptor {
        name: f.name.to_string(),
        label: f.label.to_string(),
        field_type: f.field_type.to_string(),
        keys: f.keys.map(|k| k.iter().map(|s| s.to_string()).collect()),
    }
}

struct ApiError(StatusCode, String);
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

async fn get_schema(Path(id): Path<String>) -> Result<Json<Value>, ApiError> {
    let archetype = registry::get_archetype("_common", &id).map_err(|e| ApiError(StatusCode::NOT_FOUND, e.to_string()))?;
    let fields: Vec<Value> = archetype
        .fields()
        .iter()
        .map(|f| {
            json!({
                "name": f.name, "label": f.label, "type": f.field_type,
                "required": f.required, "section": f.section, "keys": f.keys,
            })
        })
        .collect();
    let filename_fields = archetype.filename_fields();
    Ok(Json(json!({
        "id": archetype.id(), "title": archetype.title(), "governance": archetype.governance(),
        "layout": archetype.layout(), "fields": fields,
        "filenameFields": { "primary": filename_fields.primary, "secondary": filename_fields.secondary },
    })))
}

#[derive(Deserialize)]
struct PreviewRequest {
    #[serde(default = "default_namespace")]
    namespace: String,
    #[serde(alias = "archetypeId")]
    archetype_id: String,
    content: Content,
}
fn default_namespace() -> String {
    "_common".to_string()
}

async fn post_preview(Json(req): Json<PreviewRequest>) -> Result<Json<Value>, ApiError> {
    // SAME renderer path the export uses (canon rule this repo already
    // states elsewhere) -- the preview is the output, not an approximation.
    match doc_builder::build(&req.namespace, &req.archetype_id, &req.content) {
        Ok((_a, tree)) => Ok(Json(json!({ "html": render_html::render_html(&tree) }))),
        Err(e) => Ok(Json(json!({ "html": "", "errors": [e.to_string()] }))),
    }
}

#[derive(Deserialize)]
struct GenerateRequest {
    #[serde(default = "default_namespace")]
    namespace: String,
    #[serde(alias = "archetypeId")]
    archetype_id: String,
    content: Content,
    #[serde(default = "default_formats")]
    formats: Vec<String>,
    /// Who this document is for -- only read when the archetype opts into
    /// the doc registry (`doc_id_type()` is `Some`); recorded on the
    /// allocated `DocRecord`, never in the ID itself (BA26091105: the
    /// visible ID defaults to role/type, not name).
    #[serde(default)]
    recipient: String,
}
fn default_formats() -> Vec<String> {
    vec!["html".to_string()]
}

async fn post_generate(State(state): State<GenerateState>, Json(req): Json<GenerateRequest>) -> Result<Json<Value>, ApiError> {
    let mut content = req.content;

    // BA26091105: an archetype that opts into the doc registry
    // (`doc_id_type()` is `Some`) gets its `doc_id` allocated here, BEFORE
    // `build()` runs, exactly like the scheduled path already does in
    // `scheduled_brief.rs` -- this is the manual "Generate" click's
    // equivalent of that same allocation, not a second scheme. Skipped
    // when the caller already supplied a `doc_id` (e.g. a retry of an
    // already-allocated generate), so this is idempotent per request.
    if let Ok(archetype) = registry::get_archetype(&req.namespace, &req.archetype_id) {
        if let Some(type_code) = archetype.doc_id_type() {
            let already_has_id = content.get("doc_id").and_then(|v| v.as_str()).map(|s| !s.is_empty()).unwrap_or(false);
            if !already_has_id {
                let today = chrono::Utc::now().date_naive();
                let doc_id = state
                    .doc_registry
                    .allocate(type_code, today, &req.archetype_id, &req.recipient, "1.0", vec![])
                    .map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                content.insert("doc_id".to_string(), Value::String(doc_id));
            }
        }
    }

    let (_archetype, tree) = doc_builder::build(&req.namespace, &req.archetype_id, &content)
        .map_err(|e| ApiError(StatusCode::UNPROCESSABLE_ENTITY, e.to_string()))?;

    let mut files = serde_json::Map::new();
    for format in &req.formats {
        let (bytes, ext): (Vec<u8>, &str) = match format.as_str() {
            "html" => (render_html::render_html(&tree).into_bytes(), "html"),
            "md" => (render_markdown::render_markdown(&tree).into_bytes(), "md"),
            "docx" => (
                render_docx::render_docx(&tree).map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
                "docx",
            ),
            "pdf" => (
                render_pdf::render_pdf(&tree).map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?,
                "pdf",
            ),
            other => return Err(ApiError(StatusCode::BAD_REQUEST, format!("unknown format \"{other}\""))),
        };
        files.insert(
            format.clone(),
            json!({
                "filename": format!("{}.{ext}", req.archetype_id),
                "bytes": bytes.len(),
                "base64": base64::engine::general_purpose::STANDARD.encode(&bytes),
            }),
        );
    }
    Ok(Json(json!({ "archetypeId": req.archetype_id, "files": files })))
}

#[derive(Deserialize)]
struct ResearchFieldRequest {
    field: FieldDescriptor,
    #[serde(default)]
    brief: Option<String>,
    #[serde(default)]
    other_field_values: Content,
}

#[derive(Serialize)]
struct ResearchFieldResponse {
    field: String,
    value: String,
}

async fn post_research_field(
    State(state): State<GenerateState>,
    Path(archetype_id): Path<String>,
    Json(req): Json<ResearchFieldRequest>,
) -> Result<Json<ResearchFieldResponse>, ApiError> {
    let value = ai_assist::research_field(
        &state.client,
        &state.groq_api_key,
        &archetype_id,
        &req.field,
        req.brief.as_deref(),
        &req.other_field_values,
    )
    .await
    .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    Ok(Json(ResearchFieldResponse { field: req.field.name, value }))
}

#[derive(Deserialize)]
struct FullDraftRequest {
    brief: String,
}

async fn post_full_draft(
    State(state): State<GenerateState>,
    Path(archetype_id): Path<String>,
    Json(req): Json<FullDraftRequest>,
) -> Result<Json<Value>, ApiError> {
    let archetype = registry::get_archetype("_common", &archetype_id).map_err(|e| ApiError(StatusCode::NOT_FOUND, e.to_string()))?;
    let fields: Vec<FieldDescriptor> = archetype.fields().iter().map(field_def_to_descriptor).collect();
    let values = ai_assist::full_draft(&state.client, &state.groq_api_key, &archetype_id, &fields, &req.brief)
        .await
        .map_err(|e| ApiError(StatusCode::BAD_GATEWAY, e.to_string()))?;
    Ok(Json(Value::Object(values)))
}
