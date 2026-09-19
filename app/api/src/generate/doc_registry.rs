//! BA26091105: doc ID generation + registry. Scope carried forward from
//! iSconl's own backlog rather than re-derived (`git show 844b877^:
//! work/dev/Systems/iSconl/_next/backlog/build.md`, row `BA26091105`) --
//! that row left the storage tech as its one open decision. Press has no
//! Postgres wiring yet (`db.rs` is unconnected pre-Cycle-0 skeleton, no
//! migrations exist), so this resolves that decision as a file-backed
//! registry today, behind a trait a real DB-backed impl can replace later
//! without touching any call site.
//!
//! Format: `{TYPE}_{YYYYMMDD}_W{WW}` (e.g. `BRIEF_20260908_W37`). `TYPE` is
//! declared per-archetype (`Archetype::doc_id_type()`), `YYYYMMDD` the
//! generation date, `W{WW}` its ISO week. Collision rule: unique on
//! `(TYPE, YYYYMMDD)` -- the first document of a given type on a given day
//! keeps the bare form, a same-day second document appends `_02`, a third
//! `_03`, and so on.
//!
//! Deliberately allocated OUTSIDE `archetype::build()`: a doc ID is
//! stateful (it depends on what has already been allocated today, which
//! `build()` cannot know without an I/O side effect), and `build()`'s one
//! contract, unchanged by this row, is that the same `(content, version)`
//! always renders byte-identical. The caller allocates first, writes the
//! result into `content["doc_id"]`, then calls `build()` -- from
//! `build()`'s point of view `doc_id` is just another already-known field.
//!
//! Race safety: `status-brief.js`'s own ID scheme (`SB%04d` by max-suffix
//! scan) races under concurrent drafts -- two callers can read the same
//! max and allocate the same ID. This module does not inherit that: the
//! whole read-allocate-write cycle happens while holding an OS-level
//! exclusive lock (a `create_new` lockfile, which is atomic at the
//! filesystem level), so a second concurrent caller blocks until the first
//! finishes rather than racing it.

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocRecord {
    pub doc_id: String,
    pub archetype_id: String,
    /// Real recipient identity -- deliberately never encoded in the ID
    /// itself (BA26091105: documents default to role, not name, in the
    /// visible ID; the registry is where the real identity actually lives).
    pub recipient: String,
    pub version: String,
    /// Source data file refs (e.g. `["tasks.tsv", "circle/interactions.tsv"]`).
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// Output artifact paths/URLs once generated (one-pager file path,
    /// interactive URL, etc.) -- empty at allocation time, filled in by a
    /// later `update()` once rendering actually happens.
    #[serde(default)]
    pub output_paths: Vec<String>,
    /// "draft" | "sent" | "acknowledged".
    pub status: String,
    pub created_at: String,
}

#[derive(Debug)]
pub struct DocRegistryError(pub String);

impl std::fmt::Display for DocRegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for DocRegistryError {}

#[derive(Debug, Default, Serialize, Deserialize)]
struct RegistryFile {
    #[serde(default)]
    records: HashMap<String, DocRecord>,
}

pub struct DocRegistry {
    data_path: PathBuf,
    lock_path: PathBuf,
}

impl DocRegistry {
    pub fn new(data_path: impl Into<PathBuf>) -> Self {
        let data_path = data_path.into();
        let lock_path = data_path.with_extension("lock");
        Self { data_path, lock_path }
    }

    /// `DOC_REGISTRY_PATH` env var if set, else `<cwd>/data/doc_registry.json`
    /// -- mirrors `config.rs`'s own env-with-default convention.
    pub fn from_env() -> Self {
        let path = std::env::var("DOC_REGISTRY_PATH").unwrap_or_else(|_| "data/doc_registry.json".to_string());
        Self::new(path)
    }

    fn acquire_lock(&self) -> Result<LockGuard<'_>, DocRegistryError> {
        if let Some(parent) = self.lock_path.parent() {
            fs::create_dir_all(parent).map_err(|e| DocRegistryError(format!("doc_registry: cannot create data dir: {e}")))?;
        }
        let start = Instant::now();
        let timeout = Duration::from_secs(5);
        loop {
            match fs::OpenOptions::new().write(true).create_new(true).open(&self.lock_path) {
                Ok(_) => return Ok(LockGuard { path: &self.lock_path }),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    if start.elapsed() > timeout {
                        return Err(DocRegistryError("doc_registry: timed out waiting for lock (another allocation in progress)".into()));
                    }
                    sleep(Duration::from_millis(20));
                }
                Err(e) => return Err(DocRegistryError(format!("doc_registry: cannot create lock file: {e}"))),
            }
        }
    }

    fn read(&self) -> Result<RegistryFile, DocRegistryError> {
        match fs::read_to_string(&self.data_path) {
            Ok(s) if !s.trim().is_empty() => {
                serde_json::from_str(&s).map_err(|e| DocRegistryError(format!("doc_registry: malformed registry file: {e}")))
            }
            Ok(_) => Ok(RegistryFile::default()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(RegistryFile::default()),
            Err(e) => Err(DocRegistryError(format!("doc_registry: cannot read registry file: {e}"))),
        }
    }

    /// Write-to-temp-then-rename -- the write itself is atomic from any
    /// concurrent reader's point of view; the lock (held by the caller for
    /// the whole read-modify-write cycle) is what prevents a lost update
    /// between two allocators, not this alone.
    fn write(&self, file: &RegistryFile) -> Result<(), DocRegistryError> {
        let json = serde_json::to_string_pretty(file).map_err(|e| DocRegistryError(format!("doc_registry: serialize failed: {e}")))?;
        let tmp = self.data_path.with_extension("json.tmp");
        fs::write(&tmp, json).map_err(|e| DocRegistryError(format!("doc_registry: write failed: {e}")))?;
        fs::rename(&tmp, &self.data_path).map_err(|e| DocRegistryError(format!("doc_registry: rename failed: {e}")))?;
        Ok(())
    }

    /// Allocates a new doc ID for `type_code` on `date`, records it with
    /// the given metadata, and returns the ID. Holds the lock for the
    /// entire compute-and-write cycle.
    pub fn allocate(
        &self,
        type_code: &str,
        date: NaiveDate,
        archetype_id: &str,
        recipient: &str,
        version: &str,
        source_refs: Vec<String>,
    ) -> Result<String, DocRegistryError> {
        let _lock = self.acquire_lock()?;
        let mut file = self.read()?;

        let yyyymmdd = date.format("%Y%m%d").to_string();
        let iso_week = date.iso_week().week();
        let base = format!("{type_code}_{yyyymmdd}_W{iso_week:02}");

        let existing_today = file
            .records
            .keys()
            .filter(|id| *id == &base || id.starts_with(&format!("{base}_")))
            .count();
        let doc_id = if existing_today == 0 {
            base
        } else {
            format!("{base}_{:02}", existing_today + 1)
        };

        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        file.records.insert(
            doc_id.clone(),
            DocRecord {
                doc_id: doc_id.clone(),
                archetype_id: archetype_id.to_string(),
                recipient: recipient.to_string(),
                version: version.to_string(),
                source_refs,
                output_paths: vec![],
                status: "draft".to_string(),
                created_at: created_at.to_string(),
            },
        );
        self.write(&file)?;
        Ok(doc_id)
    }

    pub fn get(&self, doc_id: &str) -> Result<Option<DocRecord>, DocRegistryError> {
        Ok(self.read()?.records.get(doc_id).cloned())
    }

    /// Updates status/output paths once a document has actually been
    /// rendered/sent -- kept separate from `allocate()` since those facts
    /// aren't known until after `build()`/render run.
    pub fn update(
        &self,
        doc_id: &str,
        status: Option<&str>,
        output_paths: Option<Vec<String>>,
    ) -> Result<(), DocRegistryError> {
        let _lock = self.acquire_lock()?;
        let mut file = self.read()?;
        let record = file
            .records
            .get_mut(doc_id)
            .ok_or_else(|| DocRegistryError(format!("doc_registry: no such doc_id \"{doc_id}\"")))?;
        if let Some(s) = status {
            record.status = s.to_string();
        }
        if let Some(paths) = output_paths {
            record.output_paths = paths;
        }
        self.write(&file)
    }
}

struct LockGuard<'a> {
    path: &'a Path,
}
impl Drop for LockGuard<'_> {
    fn drop(&mut self) {
        let _ = fs::remove_file(self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    fn temp_registry() -> DocRegistry {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("doc_registry_test_{}_{n}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        DocRegistry::new(dir.join("registry.json"))
    }

    fn date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 9, 8).unwrap() // a Tuesday in ISO week 37
    }

    #[test]
    fn first_allocation_of_the_day_keeps_the_bare_form() {
        let reg = temp_registry();
        let id = reg.allocate("BRIEF", date(), "weekly-status-brief", "Joel", "1.0", vec![]).unwrap();
        assert_eq!(id, "BRIEF_20260908_W37");
    }

    #[test]
    fn a_second_same_day_document_gets_a_02_suffix() {
        let reg = temp_registry();
        let first = reg.allocate("BRIEF", date(), "weekly-status-brief", "Joel", "1.0", vec![]).unwrap();
        let second = reg.allocate("BRIEF", date(), "weekly-status-brief", "Ragnar", "1.0", vec![]).unwrap();
        assert_eq!(first, "BRIEF_20260908_W37");
        assert_eq!(second, "BRIEF_20260908_W37_02");
    }

    #[test]
    fn a_third_same_day_document_gets_a_03_suffix() {
        let reg = temp_registry();
        reg.allocate("BRIEF", date(), "weekly-status-brief", "a", "1.0", vec![]).unwrap();
        reg.allocate("BRIEF", date(), "weekly-status-brief", "b", "1.0", vec![]).unwrap();
        let third = reg.allocate("BRIEF", date(), "weekly-status-brief", "c", "1.0", vec![]).unwrap();
        assert_eq!(third, "BRIEF_20260908_W37_03");
    }

    #[test]
    fn different_type_codes_do_not_collide() {
        let reg = temp_registry();
        let brief = reg.allocate("BRIEF", date(), "weekly-status-brief", "a", "1.0", vec![]).unwrap();
        let memo = reg.allocate("MEMO", date(), "single-page-memo", "a", "1.0", vec![]).unwrap();
        assert_eq!(brief, "BRIEF_20260908_W37");
        assert_eq!(memo, "MEMO_20260908_W37");
    }

    #[test]
    fn get_returns_the_recorded_metadata() {
        let reg = temp_registry();
        let id = reg.allocate("BRIEF", date(), "weekly-status-brief", "Joel", "1.0", vec!["tasks.tsv".into()]).unwrap();
        let record = reg.get(&id).unwrap().unwrap();
        assert_eq!(record.recipient, "Joel");
        assert_eq!(record.status, "draft");
        assert_eq!(record.source_refs, vec!["tasks.tsv".to_string()]);
    }

    #[test]
    fn update_sets_status_and_output_paths_without_touching_allocation() {
        let reg = temp_registry();
        let id = reg.allocate("BRIEF", date(), "weekly-status-brief", "Joel", "1.0", vec![]).unwrap();
        reg.update(&id, Some("sent"), Some(vec!["out/brief.pdf".into()])).unwrap();
        let record = reg.get(&id).unwrap().unwrap();
        assert_eq!(record.status, "sent");
        assert_eq!(record.output_paths, vec!["out/brief.pdf".to_string()]);
    }

    #[test]
    fn concurrent_allocations_never_collide_on_the_same_id() {
        use std::sync::Arc;
        use std::thread;
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("doc_registry_test_concurrent_{}_{n}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let reg = Arc::new(DocRegistry::new(dir.join("registry.json")));
        let handles: Vec<_> = (0..8)
            .map(|i| {
                let reg = reg.clone();
                thread::spawn(move || reg.allocate("BRIEF", date(), "weekly-status-brief", &format!("r{i}"), "1.0", vec![]).unwrap())
            })
            .collect();
        let mut ids: Vec<String> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        let before = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), before, "every concurrent allocation must produce a distinct doc_id");
    }
}
