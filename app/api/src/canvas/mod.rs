//! Canvas (BB26091502): a content-type-agnostic, live-editable HTML
//! artifact. See `service.rs` for the model/persistence and `handlers.rs`
//! for the HTTP surface (creator CRUD + the unauthenticated public share
//! route).

pub mod handlers;
pub mod service;
