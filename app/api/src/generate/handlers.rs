//! HTTP surface for the generation engine (BB26091205) -- `/archetypes/*`.
//! Thin per the canon's own framing (D-009/"Thin-Client Contract"): every
//! handler here is registry lookup + validate + build + render, the same
//! pipeline `doc_builder::build` and each `render_*` module already
//! provide. No business logic lives here that doesn't already live in
//! `generate/`.
//!
//! **The request/response shapes below are NOT this file's own invention.**
//! `app/web/lib/archetypes.ts` (BB26091204, shipped first, deliberately
//! written ahead of this route existing) already declares the exact
//! contract its Creator Studio wizard calls: `PreviewResponse` (`html` +
//! `markdown`), `GenerateResponse` (`archetypeId` + `files: Record<string,
//! GeneratedFile>`), `GeneratedFile` (`filename`/`bytes`/`base64`), and a
//! request body of `{namespace, archetypeId, content}` (preview) or
//! `{namespace, archetypeId, formats: string[], content}` (generate).
//! Every handler here matches that file field-for-field rather than
//! inventing a parallel shape -- `iSconl hub`'s own Writer wizard
//! (`web/static/app.js` in that repo) is the second, independent
//! consumer and is written to this same shape.
//!
//! Every route requires a D-010 thin-client token (`auth::middleware`)
//! carrying the scope the route needs.

use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::Utc;
use serde::Deserialize;
use serde_json::{json, Map, Value};

use crate::auth::middleware::{require_scope, ThinClientAuth};
use crate::auth::service::{SCOPE_ARCHETYPES_GENERATE, SCOPE_ARCHETYPES_PREVIEW, SCOPE_ARCHETYPES_READ};
use crate::config::Config;

use super::archetype::Archetype;
use super::content::Content;
use super::doc_builder::{self, GenerateError};
use super::naming::{self, FilenameOptions};
use super::node_tree::DocumentTree;
use super::registry::{get_archetype, list_archetypes};
use super::render_docx::render_docx;
use super::render_html::render_html;
use super::render_markdown::render_markdown;
use super::render_pdf::render_pdf;

const DEFAULT_NAMESPACE: &str = "_common";
/// `app/web/lib/archetypes.ts`'s `OutputFormat` union -- the only format
/// strings `POST /archetypes/generate` accepts. Doubles as each format's
/// file extension (`naming::filename`'s `ext`), which is why "markdown"
/// is spelled "md" here, matching the TS side exactly rather than the
/// longer name `render_markdown.rs`'s own function uses internally.
const VALID_FORMATS: [&str; 4] = ["md", "docx", "pdf", "html"];

type ApiError = (StatusCode, Json<Value>);

fn err(status: StatusCode, message: impl Into<String>) -> ApiError {
    (status, Json(json!({ "error": message.into() })))
}

/// Mounted separately from any other router (`main.rs` merges it in) so
/// its state stays `Config` alone -- the thin-client extractor only needs
/// the signing secret, not a DB pool or anything else Week 2 adds later.
pub fn router(config: Config) -> Router {
    Router::new()
        .route("/archetypes", get(list_archetypes_handler))
        .route("/archetypes/preview", post(preview))
        .route("/archetypes/generate", post(generate))
        .route("/archetypes/:id/schema", get(schema))
        .with_state(config)
}

// Key names below are camelCase (`filenameFields`, `type` not
// `field_type`) rather than this codebase's usual snake_case, on purpose:
// both known callers (`app/web/lib/archetypes.ts`'s `FieldDef`/
// `ArchetypeSchema`, and iSconl hub's Writer wizard, inherited from
// `scope/lib/generate/registry.js`'s own JSON) already read exactly this
// shape.
fn archetype_summary(a: &dyn Archetype) -> Value {
    let ff = a.filename_fields();
    json!({
        "id": a.id(),
        "title": a.title(),
        "governance": a.governance(),
        "layout": a.layout(),
        "filenameFields": { "primary": ff.primary, "secondary": ff.secondary },
    })
}

fn field_summary(a: &dyn Archetype) -> Value {
    let fields: Vec<Value> = a
        .fields()
        .iter()
        .map(|f| {
            json!({
                "name": f.name,
                "label": f.label,
                "type": f.field_type,
                "required": f.required,
                "section": f.section,
                "keys": f.keys,
            })
        })
        .collect();
    Value::Array(fields)
}

#[derive(Debug, Deserialize)]
pub struct NamespaceQuery {
    pub namespace: Option<String>,
}

/// GET /archetypes?namespace= -- list archetypes available to a namespace,
/// each with its summary (not its full field schema -- see GET
/// /archetypes/:id/schema for that, per D-009's field-def-only contract
/// so a listing call stays cheap).
async fn list_archetypes_handler(
    ThinClientAuth(claims): ThinClientAuth,
    Query(q): Query<NamespaceQuery>,
) -> Result<Json<Value>, ApiError> {
    require_scope(&claims, SCOPE_ARCHETYPES_READ)?;
    let namespace = q.namespace.as_deref().unwrap_or(DEFAULT_NAMESPACE);
    let archetypes: Vec<Value> = list_archetypes(namespace)
        .iter()
        .map(|a| archetype_summary(a.as_ref()))
        .collect();
    Ok(Json(json!({ "namespace": namespace, "archetypes": archetypes })))
}

/// GET /archetypes/:id/schema?namespace= -- field definitions, for host-
/// rendered forms. This is the endpoint the acceptance criteria's "iSconl
/// wizard renders against /archetypes/:id/schema with no local copy" is
/// about -- the field list here IS the schema, nothing else declares it.
async fn schema(
    ThinClientAuth(claims): ThinClientAuth,
    Path(id): Path<String>,
    Query(q): Query<NamespaceQuery>,
) -> Result<Json<Value>, ApiError> {
    require_scope(&claims, SCOPE_ARCHETYPES_READ)?;
    let namespace = q.namespace.as_deref().unwrap_or(DEFAULT_NAMESPACE);
    let archetype =
        get_archetype(namespace, &id).map_err(|e| err(StatusCode::NOT_FOUND, e.to_string()))?;
    let mut out = archetype_summary(archetype.as_ref());
    out["fields"] = field_summary(archetype.as_ref());
    Ok(Json(out))
}

fn default_namespace() -> String {
    DEFAULT_NAMESPACE.to_string()
}

#[derive(Debug, Deserialize)]
pub struct PreviewRequest {
    #[serde(default = "default_namespace")]
    pub namespace: String,
    /// `alias`: both known callers send `archetypeId` (camelCase) in the
    /// request body, matching `app/web/lib/archetypes.ts`'s field name --
    /// accept both rather than the wire shape being snake_case only.
    #[serde(alias = "archetypeId")]
    pub archetype_id: String,
    pub content: Content,
}

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    #[serde(default = "default_namespace")]
    pub namespace: String,
    #[serde(alias = "archetypeId")]
    pub archetype_id: String,
    pub content: Content,
    /// `app/web/lib/archetypes.ts`'s `OutputFormat[]` -- one call can
    /// build several formats at once (that's the whole reason this isn't
    /// shaped like `preview`'s single-format request).
    pub formats: Vec<String>,
    /// Not part of either known caller's request today (neither sends
    /// it) -- accepted and defaulted rather than rejected, so a future
    /// caller that DOES want D-009's "optionally publish" doesn't need a
    /// route change to get it. See this handler's own doc comment for
    /// why `true` doesn't do anything yet.
    #[serde(default)]
    pub publish: bool,
}

/// Builds `namespace`/`archetype_id`/`content` into a document tree,
/// translating registry/validation failures into the response shape this
/// API uses everywhere else. A missing-field build error comes back as
/// `errors: string[]`, one entry per field, per the canon's own
/// acceptance criteria ("returned per-field, not as one opaque failure")
/// AND matching `app/web/lib/archetypes.ts`'s `normalizeErrors`, which
/// already parses exactly this per-field string shape
/// (`"missing required field: X"`) via regex. `BuildError`'s message is
/// already `"{archetype-id}: {msg1}; {msg2}"` (every archetype's
/// `build()` formats it this way, see `generate/archetypes/*.rs`), so
/// this splits on that shape rather than inventing a new one.
fn build_or_error(
    namespace: &str,
    archetype_id: &str,
    content: &Content,
) -> Result<(std::sync::Arc<dyn Archetype>, DocumentTree), ApiError> {
    doc_builder::build(namespace, archetype_id, content).map_err(|e| match e {
        GenerateError::Registry(e) => err(StatusCode::NOT_FOUND, e.to_string()),
        GenerateError::Build(e) => {
            let errors: Vec<&str> = e
                .0
                .splitn(2, ": ")
                .nth(1)
                .unwrap_or(&e.0)
                .split("; ")
                .collect();
            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": "validation failed", "errors": errors })),
            )
        }
    })
}

/// POST /archetypes/preview -- build + render, return without persisting.
/// Renders BOTH html and markdown from the same tree (cheap -- both are
/// pure functions over `DocumentTree`) since `PreviewResponse` declares
/// both and different callers use different ones: the web Creator Studio
/// renders `html` (the same renderer `generate` uses, "so the preview is
/// the output rather than an approximation of it" per that route's own
/// comment); iSconl hub's Writer preview pane feeds `markdown` through a
/// client-side markdown renderer instead.
async fn preview(
    ThinClientAuth(claims): ThinClientAuth,
    Json(req): Json<PreviewRequest>,
) -> Result<Json<Value>, ApiError> {
    require_scope(&claims, SCOPE_ARCHETYPES_PREVIEW)?;
    let (_archetype, tree) = build_or_error(&req.namespace, &req.archetype_id, &req.content)?;
    Ok(Json(json!({
        "archetypeId": req.archetype_id,
        "namespace": req.namespace,
        "html": render_html(&tree),
        "markdown": render_markdown(&tree),
    })))
}

fn render_bytes(tree: &DocumentTree, format: &str) -> Result<Vec<u8>, ApiError> {
    match format {
        "html" => Ok(render_html(tree).into_bytes()),
        "md" => Ok(render_markdown(tree).into_bytes()),
        "docx" => render_docx(tree)
            .map_err(|e| err(StatusCode::INTERNAL_SERVER_ERROR, format!("docx render failed: {e}"))),
        "pdf" => render_pdf(tree)
            .map_err(|e| err(StatusCode::INTERNAL_SERVER_ERROR, format!("pdf render failed: {e}"))),
        other => Err(err(
            StatusCode::BAD_REQUEST,
            format!("unknown format \"{other}\" -- expected one of {VALID_FORMATS:?}"),
        )),
    }
}

/// POST /archetypes/generate -- build + render every requested format +
/// optionally publish.
///
/// **`publish` is accepted but not yet actionable**: persistence
/// (`posts`/`publications`) is still Week 2 scope (see those modules'
/// own `Not yet built` doc comments) and this row does not build it. A
/// `publish: true` request still builds and renders successfully -- it
/// comes back with `"published": false` and a `publishNote` explaining
/// why, rather than either silently discarding the flag or lying about a
/// persistence step that doesn't exist. Neither known caller sends
/// `publish` today, so this never fires in practice yet. Follow-up: wire
/// this to `posts`/`publications` once their DB layer lands.
async fn generate(
    ThinClientAuth(claims): ThinClientAuth,
    Json(req): Json<GenerateRequest>,
) -> Result<Json<Value>, ApiError> {
    require_scope(&claims, SCOPE_ARCHETYPES_GENERATE)?;
    if req.formats.is_empty() {
        return Err(err(StatusCode::BAD_REQUEST, "\"formats\" must list at least one output format"));
    }
    let (archetype, tree) = build_or_error(&req.namespace, &req.archetype_id, &req.content)?;

    let ff = archetype.filename_fields();
    let mut files = Map::new();
    for format in &req.formats {
        let bytes = render_bytes(&tree, format)?;
        let filename = naming::filename(
            archetype.id(),
            (ff.primary, ff.secondary),
            &req.content,
            &FilenameOptions { version: "0.0.0", date: Utc::now().date_naive(), ext: format },
        )
        .unwrap_or_else(|_| format!("{}.{format}", archetype.id()));
        files.insert(
            format.clone(),
            json!({ "filename": filename, "bytes": bytes.len(), "base64": STANDARD.encode(&bytes) }),
        );
    }

    let mut out = json!({
        "archetypeId": req.archetype_id,
        "namespace": req.namespace,
        "files": Value::Object(files),
    });
    if req.publish {
        out["published"] = json!(false);
        out["publishNote"] = json!(
            "publish=true accepted but not yet actionable: publications persistence \
             is not built yet (publications/handlers.rs, posts/handlers.rs)"
        );
    }
    Ok(Json(out))
}
