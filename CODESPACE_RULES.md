# Codespace Rules

Conventions for every file produced in QSpace Press sessions, ported from the canon doc (`work/_arc/qspace-press/canon-canvas/20260325_..._v1_0_0.md`, Appendix G).

1. **File path comment on line 1** — every TypeScript file begins with `// app/web/path/to/file.ts`; every Rust file begins with `// app/api/src/path/to/file.rs`. (Paths updated from the canon doc's own `apps/` to this repo's actual `app/` naming, per `BO26090901`.)
2. **CHANGELOG block** — every file has an internal `// CHANGELOG:` documenting changes.
3. **CONFIG BLOCK at top** — all tunable values extracted to a clearly labelled config section near the top of each file.
4. **Comments explain why, not what** — intent, trade-offs, decisions; not restatements of code.
5. **Complete files only** — no partial snippets or ellipsis-truncated code.
6. **Changelog entry on every meaningful change** — each edit increments the file's internal changelog.

## Application to QSpace Press

- `app/web/lib/api-client.ts` — `API_BASE_URL`, `DEFAULT_TIMEOUT_MS` are the CONFIG BLOCK.
- `app/web/app/(publication)/[handle]/[slug]/page.tsx` — `REVALIDATE_SECONDS`, `MAX_EXCERPT_LENGTH` are the CONFIG BLOCK.
- `app/api/src/config.rs` — all environment variable loading; single source of truth for all config values.
- `app/api/src/payments/mpesa.rs` — `MPESA_TOKEN_URL`, `MPESA_STK_PUSH_URL`, `STK_TIMEOUT_SECONDS` are the CONFIG BLOCK.
- `app/api/src/email/worker.rs` — `BATCH_SIZE`, `BATCH_DELAY_MS`, `MAX_RETRY_ATTEMPTS` are the CONFIG BLOCK.
- `app/web/components/editor/Editor.tsx` — `AUTOSAVE_DEBOUNCE_MS`, `MAX_IMAGE_SIZE_MB` are the CONFIG BLOCK.

## Known gap, flagged not hidden

The Pre-Cycle 0 scaffold committed in `BO26090901`/`BB26090901`/`BB26090903` — before this file existed — does **not** yet follow rules 1-3 and 6 (no file-path-comment line, no CHANGELOG block, no explicit CONFIG BLOCK section). Retrofitting the existing scaffold is a separate task, not silently done as part of committing this file; whoever picks that up should treat it as its own backlog row rather than assume it's covered here. Every file written **from this commit forward** should follow these rules.
