//! BB26091203: the document-generation engine, natively rewritten in Rust
//! per Sconl's decision on BP26091402 (option a) -- not a line-by-line
//! port, a from-scratch reimplementation matching iSconl `scope`'s JS
//! engine's behavior field-for-field. See each submodule's doc comment
//! for what it's ported from and any deliberate Rust-idiom departures.
//!
//! Not wired to any HTTP route yet -- BB26091205 (thin-client API) is what
//! exposes this over `/archetypes/*`.

pub mod archetype;
pub mod archetypes;
pub mod content;
pub mod doc_builder;
pub mod naming;
pub mod node_tree;
pub mod registry;
// render_docx/render_pdf land in a follow-up checkpoint (docx-rs/genpdf
// crate integration, verified against a real cargo build) -- not declared
// yet so the rest of this module compiles and tests cleanly on its own.
pub mod render_html;
pub mod render_markdown;
pub mod style;
