//! BB26091208: a thin library target alongside the existing `main.rs`
//! binary, added only so `examples/render_weekly_status_brief_sample.rs`
//! (this row's end-to-end verification helper) can `use
//! qspace_press_api::generate::...` -- an example target can only depend
//! on a library crate, not a binary's own inline `mod` tree. `main.rs`
//! keeps its own `mod generate;` unchanged; both simply compile the same
//! `src/generate/` sources as two independent targets, same as any crate
//! with both a `lib.rs` and a `main.rs`.
pub mod generate;
