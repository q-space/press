//! Structured List Documents v1 (BP26091902) -- a new QSpace document kind,
//! not a Canvas extension. Design pass: `PP26091901`.
//!
//! ## 1. A new document kind, not a Canvas extension
//!
//! Canvas's `content_html` is a single opaque blob by design -- that is what
//! makes contentEditable simple. Giving Canvas per-item rows too would make
//! every Canvas read/write path branch on "is this plain HTML or does it
//! have items," contaminating the simple case to serve the complex one.
//! Structured lists share the archetype and share-link *machinery* with
//! Canvas, never its storage model. See `migrations/004_structured_lists.sql`.
//!
//! ## 2. Multi-editor semantics -- per-item optimistic locking, not CRDT
//!
//! A PATCH to one item carries the `version` it was read at. The write is
//! accepted only if that still matches the stored version; otherwise it is
//! rejected with the current row so the caller can reload and retry. See
//! `service::apply_patch`. Two people editing different items never
//! collide, since the lock is per-row. Two people editing the same item
//! within seconds is rare for this use case and an honest 409 is an
//! acceptable v1 failure mode, not a defect to engineer around further.
//!
//! ## 3. Auth on the public link
//!
//! A structured list's share link is the same unauthenticated mechanism
//! Canvas already uses, extended with a `can_edit` capability flag scoped
//! to the *link*, not the document -- see `structured_list_share_links` in
//! the migration. Default `can_edit = false` (read-only, matching Canvas's
//! current default); an owner explicitly turns on collaborative editing to
//! generate an editable link. This avoids building real per-viewer accounts
//! for reviewers/contractors who don't have QSpace logins and shouldn't
//! need them.
//!
//! ## Not yet wired
//!
//! `main.rs` does not connect to Postgres for ANY module yet (see `db.rs`'s
//! own doc comment -- that lands once SeaORM entities are generated from
//! the migrations). This module is therefore data model + pure business
//! logic only, ready to wire into the router in one pass once `db.rs` is
//! connected -- the same stage `payments` and `posts` are at.
//!
//! ## Not scoped here, deliberately
//!
//! Real-time collaborative cursors/presence, drag-and-drop reordering
//! beyond a plain `position` integer, and per-viewer identity beyond the
//! free-text `owner_name` -- see `PP26091901` for the reasoning.

pub mod model;
pub mod service;
pub mod handlers;
