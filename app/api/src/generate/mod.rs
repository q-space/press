//! BB26091203: the document-generation engine, natively rewritten in Rust
//! per Sconl's decision on BP26091402 (option a) -- not a line-by-line
//! port, a from-scratch reimplementation matching iSconl `scope`'s JS
//! engine's behavior field-for-field. See each submodule's doc comment
//! for what it's ported from and any deliberate Rust-idiom departures.
//!
//! BB26091205 wires this over `/archetypes/*` -- see `handlers::router`.

pub mod archetype;
pub mod archetypes;
pub mod content;
pub mod doc_builder;
pub mod handlers;
pub mod naming;
pub mod node_tree;
pub mod registry;
pub mod render_docx;
pub mod render_html;
pub mod render_markdown;
pub mod render_pdf;
pub mod style;
