# L — Light

<!-- Low-risk, mechanical work safe for a lighter model to execute
     unattended. Holds two subsets — Refine and Organize — see
     STANDING-RULES.md "What a Light session does" and "Subset
     structure" for the full mechanics. -->

## Subset: Refine (R)

<!-- Non-critical UI/UX polish — nothing broken, just not as good as it
     could be. -->

| # | Title | Task | Status | Notes |
|---|-------|------|--------|-------|
<!-- BO26091501 (Refine subset) SHIPPED 15 Sep 2026 -> done.md, the actionable part (canon write, both projects). Confirming the desktop targets build and deciding app/mobile's rename remain explicitly deferred to "when the scaffold is next touched" -- not a gap, the row's own scoping. -->

## Subset: Organize (O)

<!-- Folder/file moves (local or remote), repo restructuring, renames,
     and refactors. See STANDING-RULES.md "What a Light session does"
     for the copy-verify-delete discipline. Content below is carried
     over verbatim from the former organize.md on the BO26091401
     migration -- no rows re-minted, no comments dropped. -->

| # | Title | Task | Status | Notes |
|---|-------|------|--------|-------|
<!-- BO26090901 CLOSED 16 Sep 2026 -> done.md. It shipped as 3e743cd on 9-10 Sep and the status cell was corrected on 12 Sep, but the row was never moved out of the category file, so a Light run kept picking it up as work.
     ORIGINAL ROW, retained for provenance:
| BO26090901 (new, renamed `apps/`→`app/` 9 Sep 2026 per Sconl) | **Restructure `q-space/press` into a `app/{web,api,mobile}` monorepo shape — move the existing Flutter mobile scaffold into `app/mobile/`, add `app/web/`, `app/api/`, `packages/shared/` alongside it. One repo, three apps, per Sconl's explicit direction (9 Sep 2026), singular `app/` not plural `apps/` per his follow-up correction same day.** `git mv` every existing top-level file/folder (`lib/`, `android/`, `ios/`, `web/` [Flutter's own, not to be confused with the new Next.js `app/web/`], `linux/`, `macos/`, `windows/`, `test/`, `pubspec.yaml`, `pubspec.lock`, `analysis_options.yaml`, `.metadata`, `README.md` — everything from the 6 May 2026 `flutter create` commit, confirmed untouched since) into a new `app/mobile/` folder, preserving git history via `git mv` not delete+recreate. Add a root `pnpm-workspace.yaml` (`packages: ["app/*", "packages/*"]` — `app/mobile` sits in the workspace tree for repo organization only; pnpm itself won't manage its Dart/Flutter deps, that's `pub`'s job, no conflict). Root `README.md` rewritten to describe the monorepo (three apps: `web` deployed now, `api` backend, `mobile` — explicitly noted as post-launch/inactive per the canon doc's own MVP exclusion list, not to be started early). Root `.gitignore` merged (Flutter's existing entries + new Next.js/Rust/Node/Docker entries the scaffold will need). | ✅ **DONE 9–10 Sep 2026, shipped as `3e743cd`** — status corrected 12 Sep 2026, this cell still read "Queued, do this FIRST" two days after it shipped. The Flutter scaffold was `git mv`'d into `app/mobile/` with history preserved (confirmed as renames, not delete+recreate), `pnpm-workspace.yaml` added, root README and `.gitignore` written. **The follow-up this row itself asked for is also now done:** every stale `apps/` reference in the canon doc was corrected to `app/` (19 occurrences) in canon v2.0.0, 12 Sep 2026. **One later addition, for the record:** the local checkout was renamed `qspace-press-repo` → `Press` on 12 Sep per Sconl, and `dev`/`staging`/`main` were created to match the fleet — see `done.md`. **Scoped 9 Sep 2026, PLAN session, per Sconl's explicit correction** to this session's own earlier assumption (a separate `qspacepress` repo) — "the web platform is the one that should use the press repo... is there a way to have both scaffolding under one repo?" Answer: yes, cleanly — the canon doc's own `apps/web/`+`apps/api/` file-structure diagram (§Technology Stack) already describes exactly this monorepo shape (just pluralized differently — folded in the `apps/`→`app/` rename Sconl asked for same day); `app/mobile/` for the Flutter companion is the same pattern extended, not a deviation from it. The existing Flutter commit (`312a999`, 6 May 2026) is real but has zero actual development in it (bare `flutter create` output, README still saying "A new Flutter project") — safe to relocate wholesale, nothing to lose by moving it. **Every `apps/` reference in the canon doc itself (file-structure diagrams, env var reference, CI workflow) is now stale against this row's `app/` naming** — flag for whoever executes this to also bump the canon doc's own version with a find-replace pass, not silently left to drift (per `CLAUDE.md` §15's "keep canon current" rule). |
-->
