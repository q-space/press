//! Library crate so `src/bin/*` (currently just `mint_token`, BB26091205)
//! can reuse the same modules the `qspace-press-api` binary (`main.rs`)
//! runs -- a second `mod auth;`-style declaration tree in a bin file
//! can't see the first one's private items, so a thin lib crate that both
//! `main.rs` and `src/bin/*.rs` depend on is the standard fix rather than
//! duplicating module code.

pub mod analytics;
pub mod audience;
pub mod auth;
pub mod config;
pub mod db;
pub mod distribution;
pub mod email;
pub mod generate;
pub mod lists;
pub mod payments;
pub mod posts;
pub mod publications;
pub mod storage;
