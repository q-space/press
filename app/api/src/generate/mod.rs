//! BB26091203: the document-generation engine, natively rewritten in Rust
//! per Sconl's decision on BP26091402 (option a) -- not a line-by-line
//! port, a from-scratch reimplementation matching iSconl `scope`'s JS
//! engine's behavior field-for-field. See each submodule's doc comment
//! for what it's ported from and any deliberate Rust-idiom departures.
//!
//! `routes.rs` wires a deliberately narrow HTTP slice over this
//! (`/archetypes/:id/schema`, `/preview`, `/generate`, the two ai-assist
//! endpoints) as of BB26091208 -- see that module's own doc comment. The
//! full `/archetypes/*` CRUD surface is still BB26091205's, not yet
//! started.

pub mod archetype;
pub mod archetypes;
pub mod ai_assist;
pub mod content;
pub mod doc_builder;
pub mod doc_registry;
pub mod naming;
pub mod node_tree;
pub mod registry;
pub mod render_docx;
pub mod render_html;
pub mod render_markdown;
pub mod render_pdf;
pub mod routes;
pub mod scheduled_brief;
pub mod style;
