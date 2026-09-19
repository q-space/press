//! Canvas persistence. Deliberately the whole model: an id, a title,
//! rendered HTML, and timestamps. No `content_type`, no `course_id`, no
//! field that presumes what produced the HTML -- see `migrations/004_canvas.sql`
//! and BB26091502's row note for why that boundary matters.
//!
//! Raw SQL via `sea_orm::Statement` rather than a generated entity: no
//! SeaORM entity layer exists anywhere in this codebase yet (`posts` and
//! `publications` are still CRUD stubs -- see their own `handlers.rs`), so
//! standing one up just for Canvas would be a bigger, unrelated lift. This
//! reads the same way an entity-backed repository would from the caller's
//! side (`Canvas`, `create`, `get`, `update`) and is a mechanical swap to
//! a real entity later if/when one gets introduced for the rest of the API.

use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, FromQueryResult, Statement};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromQueryResult)]
pub struct Canvas {
    pub id: Uuid,
    pub title: String,
    pub content_html: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// What any caller hands Canvas to publish: a title and rendered HTML.
/// Nothing else -- see the module doc comment.
#[derive(Debug, Deserialize)]
pub struct CreateCanvas {
    pub title: String,
    pub html: String,
}

/// A live edit. Both fields optional so a title-only or html-only PUT
/// (the editor autosaves html on every debounced keystroke; title
/// changes separately) never clobbers the field it didn't touch.
#[derive(Debug, Deserialize, Default)]
pub struct UpdateCanvas {
    pub title: Option<String>,
    pub html: Option<String>,
}

const SELECT_COLUMNS: &str = "id, title, content_html, created_at, updated_at";

pub async fn create(db: &DatabaseConnection, input: CreateCanvas) -> Result<Canvas, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        format!(
            "INSERT INTO canvases (title, content_html) VALUES ($1, $2) RETURNING {SELECT_COLUMNS}"
        ),
        [input.title.into(), input.html.into()],
    );
    Canvas::find_by_statement(stmt)
        .one(db)
        .await?
        .ok_or_else(|| DbErr::Custom("canvas insert returned no row".into()))
}

pub async fn get(db: &DatabaseConnection, id: Uuid) -> Result<Option<Canvas>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        format!("SELECT {SELECT_COLUMNS} FROM canvases WHERE id = $1"),
        [id.into()],
    );
    Canvas::find_by_statement(stmt).one(db).await
}

pub async fn update(
    db: &DatabaseConnection,
    id: Uuid,
    input: UpdateCanvas,
) -> Result<Option<Canvas>, DbErr> {
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        format!(
            "UPDATE canvases \
             SET title = COALESCE($2, title), \
                 content_html = COALESCE($3, content_html), \
                 updated_at = NOW() \
             WHERE id = $1 \
             RETURNING {SELECT_COLUMNS}"
        ),
        [id.into(), input.title.into(), input.html.into()],
    );
    Canvas::find_by_statement(stmt).one(db).await
}
