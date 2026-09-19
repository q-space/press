//! BB26091208 verification helper -- builds a realistic
//! `weekly-status-brief` end to end (doc ID allocation -> `build()` ->
//! HTML render) and writes the HTML to disk, so it can be fed to
//! `app/web/scripts/render-pdf-cdp.mjs` for a real PDF without needing the
//! full HTTP server + a Groq key. Deliberately bypasses `ai_assist` (no
//! network call) -- this is a fixed, realistic content fixture, not a
//! test of the AI-assist path (that's `ai_assist.rs`'s own unit tests).
//!
//!   cargo run --example render_weekly_status_brief_sample

use qspace_press_api::generate::doc_builder;
use qspace_press_api::generate::doc_registry::DocRegistry;
use qspace_press_api::generate::render_html;
use serde_json::json;
use std::fs;

fn main() {
    let out_dir = std::env::temp_dir().join("bb26091208-verify");
    fs::create_dir_all(&out_dir).expect("create out dir");

    let registry = DocRegistry::new(out_dir.join("doc_registry.json"));
    let week_of = chrono::NaiveDate::from_ymd_opt(2026, 9, 14).unwrap(); // a Monday, ISO week 38
    let doc_id = registry
        .allocate(
            "BRIEF",
            week_of,
            "weekly-status-brief",
            "Joel",
            "1.0",
            vec!["tasks.tsv".into(), "circle/interactions.tsv".into()],
        )
        .expect("allocate doc id");
    println!("allocated doc_id: {doc_id}");

    let content = json!({
        "subject_id": "sconl",
        "audience": "Joel",
        "week_of": "2026-09-14",
        "version": "1.0",
        "author": "Sconl Peter",
        "doc_id": doc_id,
        "signal": [
            "Weekly-status-brief archetype shipped on the node-tree engine.",
            "Field-level AI assist wired end to end against Groq.",
            "Friday auto-draft rebuilt as a real scheduled job.",
        ],
        "substance": [
            "Doc ID registry allocates race-safely under an OS-level lock.",
            "PDF verified one-page via CDP printToPDF, not the Chrome CLI.",
            "Every list-shaped field capped at 3 items structurally, not by convention.",
        ],
        "trajectory": [
            "Wire the full BB26091205 archetype API surface.",
            "Move the doc registry onto Postgres once migrations exist.",
        ],
        "risks_blockers": [
            "No DB is wired yet -- the registry is file-backed until Postgres lands.",
        ],
        "anticipated_qa": [
            {"question": "Why file-backed and not Postgres?", "answer": "No migrations exist yet; the trait swaps in a DB-backed impl later without touching call sites."},
            {"question": "Does the scheduler bypass the archetype?", "answer": "No -- it calls the same doc_builder::build() a manual Generate click uses."},
        ],
        "next_brief_date": "2026-09-21",
    });
    let content = content.as_object().unwrap().clone();

    let (_archetype, tree) = doc_builder::build("_common", "weekly-status-brief", &content).expect("build weekly-status-brief");
    let html = render_html::render_html(&tree);

    let html_path = out_dir.join("weekly-status-brief-sample.html");
    fs::write(&html_path, &html).expect("write html");
    println!("wrote {}", html_path.display());
    println!("next: node app/web/scripts/render-pdf-cdp.mjs {} {}", html_path.display(), out_dir.join("weekly-status-brief-sample.pdf").display());
}
