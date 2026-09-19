//! HTTP surface for the `generate::` engine (BP26091906). The engine
//! (archetypes, renderers, doc_builder) already existed and compiled --
//! `main.rs` routed nothing to it. This is the missing serving layer, not
//! new generation logic.
//!
//! ## The contract
//!
//! Stateless by design, per the row's own instruction: the caller (iSconl)
//! resolves engagement/venture identity, indexes `generated_docs.tsv`,
//! pushes to OneDrive and binds `tasks.tsv` entries -- none of that is
//! Press's concern. Press's job is exactly: given a resolved `namespace` +
//! `archetype_id` + `content`, produce one rendered document and hand back
//! its bytes. No side effects, no storage, nothing kept after the response.
//!
//! `POST /api/generate` -- body: `{namespace, archetype_id, content,
//! format?, version?}` (format defaults to "html"; version defaults to
//! "1.0.0", feeding the filename convention naming.rs already implements).
//! Returns the rendered document as the raw response body with the
//! correct `Content-Type` and a `Content-Disposition: attachment;
//! filename="..."` header built from naming.rs's own convention -- a
//! caller gets a file, not a base64 string to decode.
//!
//! `GET /api/generate/archetypes?namespace=...` -- introspection: every
//! archetype's id/title/governance/fields, so a caller can build a form
//! (or validate a payload) without hardcoding the schema on its own side.

use axum::extract::Query;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use serde::{Deserialize, Serialize};

use super::content::Content;
use super::doc_builder;
use super::naming::{self, FilenameOptions};
use super::registry::list_archetypes;
use super::render_docx::render_docx;
use super::render_html::render_html;
use super::render_markdown::render_markdown;
use super::render_pdf::render_pdf;

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    #[serde(default = "default_namespace")]
    pub namespace: String,
    pub archetype_id: String,
    pub content: Content,
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_version")]
    pub version: String,
}

fn default_namespace() -> String {
    "_common".to_string()
}
fn default_format() -> String {
    "html".to_string()
}
fn default_version() -> String {
    "1.0.0".to_string()
}

#[derive(Debug, Serialize)]
struct ErrorBody {
    error: String,
}

fn error_response(status: StatusCode, message: impl Into<String>) -> Response {
    (status, Json(ErrorBody { error: message.into() })).into_response()
}

/// `POST /api/generate`. Renders one document and returns it as the raw
/// response body -- never JSON-wrapped/base64-encoded, so a caller can
/// stream it straight to a file.
pub async fn generate_document(Json(req): Json<GenerateRequest>) -> Response {
    let (ext, content_type): (&str, &str) = match req.format.as_str() {
        "html" => ("html", "text/html; charset=utf-8"),
        "markdown" | "md" => ("md", "text/markdown; charset=utf-8"),
        "docx" => (
            "docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        ),
        "pdf" => ("pdf", "application/pdf"),
        other => {
            return error_response(
                StatusCode::BAD_REQUEST,
                format!("unsupported format \"{other}\" -- expected html, markdown, docx, or pdf"),
            )
        }
    };

    let (archetype, tree) = match doc_builder::build(&req.namespace, &req.archetype_id, &req.content) {
        Ok(pair) => pair,
        Err(e) => {
            // Registry misses (unknown archetype) are a client error distinct
            // from a build failure (invalid content against a real
            // archetype) -- 404 vs 400, so a caller can tell "you asked for
            // something that doesn't exist" from "what you sent was wrong".
            let msg = e.to_string();
            let status = if msg.contains("no archetype") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::BAD_REQUEST
            };
            return error_response(status, msg);
        }
    };

    let filename_opts = FilenameOptions {
        version: req.version.clone(),
        date: Utc::now().date_naive(),
        ext,
    };
    let ff = archetype.filename_fields();
    let filename = match naming::filename(
        archetype.id(),
        (ff.primary, ff.secondary),
        &req.content,
        &filename_opts,
    ) {
        Ok(f) => f,
        Err(e) => return error_response(StatusCode::BAD_REQUEST, e.to_string()),
    };

    let bytes: Vec<u8> = match req.format.as_str() {
        "html" => render_html(&tree).into_bytes(),
        "markdown" | "md" => render_markdown(&tree).into_bytes(),
        "docx" => match render_docx(&tree) {
            Ok(b) => b,
            Err(e) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        },
        "pdf" => match render_pdf(&tree) {
            Ok(b) => b,
            Err(e) => return error_response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        },
        _ => unreachable!("format already validated above"),
    };

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, content_type.to_string()),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            ),
        ],
        bytes,
    )
        .into_response()
}

#[derive(Debug, Deserialize)]
pub struct ArchetypeListQuery {
    #[serde(default = "default_namespace")]
    pub namespace: String,
}

#[derive(Debug, Serialize)]
struct FieldOut {
    name: &'static str,
    label: &'static str,
    field_type: &'static str,
    required: bool,
    section: Option<&'static str>,
    keys: Option<&'static [&'static str]>,
}

#[derive(Debug, Serialize)]
struct ArchetypeOut {
    id: &'static str,
    title: &'static str,
    governance: bool,
    filename_fields: (&'static str, &'static str),
    fields: Vec<FieldOut>,
}

/// `GET /api/generate/archetypes?namespace=...` -- schema introspection so
/// a caller does not have to hardcode each archetype's field shape.
pub async fn archetypes(Query(q): Query<ArchetypeListQuery>) -> Json<Vec<ArchetypeOut>> {
    let out = list_archetypes(&q.namespace)
        .into_iter()
        .map(|a| {
            let ff = a.filename_fields();
            ArchetypeOut {
                id: a.id(),
                title: a.title(),
                governance: a.governance(),
                filename_fields: (ff.primary, ff.secondary),
                fields: a
                    .fields()
                    .iter()
                    .map(|f| FieldOut {
                        name: f.name,
                        label: f.label,
                        field_type: f.field_type,
                        required: f.required,
                        section: f.section,
                        keys: f.keys,
                    })
                    .collect(),
            }
        })
        .collect();
    Json(out)
}
