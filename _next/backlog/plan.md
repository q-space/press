# P — Plan

<!-- GENERATED from _next/rows/ by _kit/bin/backlog-render.js -- DO NOT EDIT.
     Your changes here are overwritten on the next run and are NOT the record.
     Edit the row file: _next/rows/<ID>.md -->

<!-- Real and wanted, but needs a design/scoping pass first — including
     anything blocked purely on the owner's input (flagged 🔴). THIS is
     the file a Plan session works from and adds to — see
     STANDING-RULES.md for what a Plan session is supposed to produce,
     the BB/BW/BP/BF/BR/BO ID format, and the sync protocol. Outflow goes
     to build.md, work.md, fix.md, refine.md, or organize.md depending on
     what the row turns out to be. -->

| # | Title | Task | Status | Notes |
|---|-------|------|--------|-------|
<!-- BP26091407 ROUTED to light.md as BO26091501 and closed here, 15 Sep 2026,
     PLAN session (relay-29).

     Same problem and same fix as relay's BP26091404, which hit this
     identically on the same day: a constraint on future work sitting in a
     queue that the sessions doing that work do not read.

     The risk this row names is not that the desktop targets are missing —
     they already exist, the app/mobile scaffold carries android, ios, linux,
     macos, windows and web platform folders. The risk is that the folder is
     called app/mobile, which teaches every session that opens it to think
     mobile-only, so a phone-shaped design gets built before anyone notices.
     That risk is live now, while the row itself could not be executed until a
     scaffold compiles.

     Splitting on that line is what unsticks it: the part that is actionable
     today — writing the requirement into the project's canon, where
     CLAUDE.md section 15 makes it read before substantive work — happens now.
     The parts that genuinely need a compiling scaffold (confirming Windows,
     macOS and Linux actually build, and choosing whether app/mobile gets
     renamed) wait for one, with the requirement already binding either way.

     Neither app/companion nor app/desktop-mobile was chosen. That stays with
     whoever builds the scaffold, deliberately.

     Carried across: the same requirement applies to qspace-pages' own
     forthcoming Flutter companion app (BB26091401), so it goes into that
     project's canon too, not only this one.

     Original row text archived below.

<!-- BP26091403 RE-MINTED BB26091501 and moved to build.md, 15 Sep 2026, PLAN
     session (relay-29), once PL26091406 resolved.

     UNBLOCKED BY: Sconl's decision that iSpark stays its own product and is
     not folded into Qpress.

     That decision does more than unblock this row — it changes what gets
     built. This row's central move was to give Canvas "a course-module
     content type alongside whatever document types the archetype system
     already handles." Course content now belongs to iSpark, so adding a
     course content type to Qpress's post model would quietly re-implement the
     fold Sconl just ruled out.

     SCOPED CONTENT-TYPE-AGNOSTIC INSTEAD, which delivers his original ask
     without that: Canvas becomes a generic live-editable HTML artifact
     surface that takes rendered HTML and a title and knows nothing about
     course modules. iSpark already reuses Qpress's rendering engine — his own
     stated direction — so it publishes course modules through Canvas the same
     way anything else does, and the Viva sharing use case is served without
     Qpress owning course content.

     THIS IS A SCOPING CALL, NOT HIS WORDS, and the build row flags it so he
     can redirect in a sentence. He asked for Canvas and for sharing course
     modules through it; generic Canvas delivers both. If he actually wants
     Qpress to own a course content type, that is a different build and the
     row needs rewriting rather than adjusting.

     The cost asymmetry is why the call was made rather than escalated: a
     generic Canvas that later gains a course type is an addition; a course
     type baked into the post model and later removed is a migration.

     UNCHANGED AND STILL CORRECT: the Aria half. Canvas belongs to Qpress
     rather than Aria, established by reading Aria's actual repo — its
     pipeline outputs .mp4, .pptx, .mp3 and a cue-sheet PDF, with no HTML or
     web-document output anywhere in its stack. Different medium entirely.

     Also corrected on the way out: this row was marked "Queued, no new infra
     needed" while silently depending on a paused positioning decision. It
     spent the day looking more ready than it was.

     Original row text archived below.

<!-- BP26091402 resolved 14 Sep 2026, PLAN session, per Sconl: option (a) --
     native Rust rewrite of the generation engine, matching BUILD's own
     recommendation (the only option consistent with canon D-002's reason
     for choosing Rust at all). Re-minted BB26091203 (was already that ID,
     re-scoped in place from "port" to "rewrite"), moved back to build.md,
     unblocked. Original text archived below.

     BP26091402 (new) 🔴 | **DECIDE: how does `BB26091203`'s "port" actually reach `app/api` — a real Rust rewrite of `render-docx`/`render-pdf`, or a JS runtime/sidecar embedded in a Rust service?** | **Bounced back from `build.md`'s `BB26091203` before any code was written — this is an unscoped architectural call, not a translation task, and the row's own premise turns out to be wrong once the target runtime is actually checked.** `BB26091203`/`BP26091201` both describe this as a "port," reasoning "the engine is already correctly factored, which is why this is a port rather than a rewrite" — true for JS→JS reuse, **not checked against what `app/api` actually is.** Confirmed live: `app/api/Cargo.toml` is a plain Axum/SeaORM Rust binary with **zero** JS-runtime crate (no `deno_core`, `boa`, `rquickjs`, `v8`, nothing) and zero document-generation crate of any kind. The two renderers that matter most, `render-docx.js` (4.9KB) and `render-pdf.js` (4.0KB), are thin wrappers around the mature JS-ecosystem libraries `docx` and `pdfkit` — there is no drop-in Rust equivalent already vetted anywhere in this codebase or canon. **The real options, genuinely different engineering efforts:** (a) hand-rewrite the ~34KB of engine/renderer/archetype JS in Rust against `docx-rs`/`printpdf`-class crates, accepting a real fidelity risk (every archetype's rendered `.docx`/`.pdf` needs re-validation against the JS output, byte-identical generation is explicitly required by `BB26091208`'s acceptance criteria) — the honest "port" in spirit, but a first-time undertaking with no existing pattern to follow in this fleet; (b) run the existing Node engine as a sidecar/microservice the Rust API calls internally, preserving the JS code and its exact output completely unchanged — fast and low-risk, but directly cuts against canon D-002's own stated reasons for choosing Rust in the first place ("binary deployment — no runtime dependency management"), and there is no existing precedent anywhere in the fleet (which is otherwise all-Node) or in Qpress's own canon for a mixed-runtime service; (c) embed a JS engine (e.g. `rquickjs`/`boa`) inside the Rust process itself — keeps deployment single-binary, but the JS `docx`/`pdfkit` libraries were never written against an embedded-engine environment (they assume Node's `fs`/`Buffer`/streams) and would need real compatibility work, unproven. | 🔴 Blocked on Sconl | **Found 14 Sep 2026, BUILD session, on `BB26091203`'s own first step** (reading `app/api`'s real Dockerfile/Cargo.toml before committing to an approach, per standing BUILD-session protocol after `BI26091110`'s precedent in iSconl). No code was written against any of the three options — this is a genuine fork in engineering approach with real cost/fidelity tradeoffs only Sconl can weigh, not a scoping gap a BUILD session should guess through. **Recommendation, not a decision:** (a) is the only option consistent with why Rust was chosen for this service at all (D-002) and is the only one that doesn't introduce an unprecedented mixed-runtime or embedded-engine pattern into a codebase that has neither — but it is real, first-time, multi-session work (rewriting DOCX/PDF binary generation from scratch, plus a validation pass per archetype), not a quick follow-on to `BB26091202`. `BB26091203` stays queued in `build.md` pointing here rather than being silently attempted under any one of these three assumptions. -->
| # | Title | Task | Status | Notes |
|---|-------|------|--------|-------|
| BP26090801 (new) | **Rename the isconl agent's "Writer" space to "Press"/"QPress" and rebuild it as a thin client into Qpress's own Publishing Engine API — blocked until that API exists** | **Not build-ready yet.** Qpress has no repo, no API, no Feature 1 (Publishing Engine) shipped — canon doc status is "Pre-Cycle 0, no commits yet." Writer today generates internal business documents (reports/briefs, archetype/wizard-driven, tied to `scope/tasks.tsv` engagements/projects, output `.docx`/`.md` via `scope/lib/generate/`) — a genuinely different shape from Qpress's Publishing Engine (SEO-indexed public posts + newsletter delivery to external subscribers, Tiptap editor). Rebuilding Writer as a thin client requires a real target API to point it at. **Do not attempt before `Feature 1: Publishing Engine` (Qpress's own Cycle 1 milestone) ships and exposes a usable API.** | 🔴 blocked | **Scoped 8 Sep 2026, iSconl PLAN session, per Sconl's explicit decision** (accepted despite the investigation finding real shape differences between Writer and Qpress's Publishing Engine — his call to proceed anyway, not re-litigated here). Cross-referenced from `PL26090801` in `Systems/iSconl/_next/backlog/plan.md` (now closed there — the 3 open questions it asked are resolved; this row and `BP26090802` below are the concrete follow-through). **Revisit this row once Qpress's Cycle 1 actually ships** — re-check at that point whether Writer's current archetype/wizard model can realistically wrap Qpress's post-editor API, or whether "thin client" needs to mean something narrower (e.g. Writer keeps its own document-generation archetypes for internal docs, and only a NEW archetype type is added that posts to Qpress) — not assumed here, a real design question once there's an API to design against. **UPDATED 12 Sep 2026, PLAN session:** premise substantially overtaken by Sconl's direction that ALL builds happen inside Qpress with iSconl as a thin client — see `BP26091201` below, which supersedes this row's framing. Also corrected: the investigation's "Writer has a Tiptap editor" premise was wrong in both directions. iSconl has **no** rich-text editor at all (no Tiptap/ProseMirror/Quill/Lexical/contenteditable anywhere in `hub`) — it has a 3-step wizard rendering an HTML form from archetype field definitions. And `hub` holds **no** generation business logic: it is already a pure HTTP client of `scope`+`spark` via 13 route mappings in `hub/lib/api-compat.js`. The "thin client" is therefore re-pointing 13 routes, not a rebuild. **REVIEWED 15 Sep 2026, PLAN session (`relay-29`) — correctly blocked, nothing to change, and nothing needed from Sconl.** The blocker is real and unchanged: Qpress's Feature 1 Publishing Engine has not shipped, so there is no API to point a thin client at. **Worth carrying forward, because this row has been partially superseded twice and the layers are easy to misread:** its original framing (rebuild Writer against Qpress's editor API) was overtaken on 12 Sep by `BP26091201`, which made iSconl a thin client of Qpress wholesale; and its own investigation premise was corrected in both directions — iSconl has **no** rich-text editor at all, and `hub` holds **no** generation logic, being already a pure HTTP client of `scope` and `spark` via 13 route mappings in `hub/lib/api-compat.js`. **So the eventual work is genuinely small**, and the row's own "revisit once Cycle 1 ships" instruction stands: re-check then whether Writer's archetype model can wrap Qpress's post-editor API, or whether thin client needs to mean something narrower. **Related and now tracked: `BP26091205` in `qspace-press` `build.md` already carries the concrete re-pointing of those 13 routes**, so this row is the design question and that row is the execution. |
| BP26091207 (new) | **Lift the `main` branch lock once `qspacepress.com` is registered** | `main` on `q-space/press` was locked read-only 12 Sep 2026 (canon D-014) so nothing can reach a production path that does not exist yet. Protection applied: `lock_branch: true`, `enforce_admins: true`, `allow_force_pushes: false`, `allow_deletions: false`. **To lift:** `gh api -X DELETE repos/q-space/press/branches/main/protection`, or edit it in the GitHub UI under Settings → Branches. **Preconditions before lifting:** (1) `BH26091202` done — domain registered and the DNS zone live; (2) a production deploy target actually exists (tunnel ingress rule + DNS record for `qspacepress.com`, same mechanism as staging); (3) a `deploy-production.yml` workflow written and reviewed — deliberately NOT written yet, since a workflow pointing at a destination that does not resolve is worse than no workflow. | 🔴 blocked | **Created 12 Sep 2026 at Sconl's request.** Note the lock is on the *branch*, not the repo: `dev` and `staging` are entirely unaffected and staging deploys continue normally. The lock is fully reversible by any admin at any time. **BLOCKER VERIFIED STILL LIVE 15 Sep 2026, PLAN session (`relay-29`): `BH26091202` is a live row in this project's `hands.md` — register `qspacepress.com`, about $10.46/year at Cloudflare Registrar, deferred until Sconl's Cloudflare account is funded. Not shipped, not quietly dropped, so this row correctly stays put.** **Nothing here needs a decision from him, only the purchase**, and that is already stated in the right place rather than duplicated here. **Worth saying plainly because a branch lock reads alarming out of context: nothing is broken and nothing is waiting on this.** The lock is on `main` only; `dev` and `staging` are unaffected and staging deploys continue normally. Its whole purpose is to stop anything reaching a production path that does not resolve yet, which is still true. **Do not lift it early to "unblock" something** — the two remaining preconditions beyond the domain (a real production deploy target, and a reviewed `deploy-production.yml`) exist because a workflow pointing at a destination that does not resolve is worse than no workflow. |
| BP26091204 (new) | **Scope the Institutional tier — the commercial case for Feature 5, and the largest projected stream by Year 2** | Canon v2.0.0 adds Institutional (from KES 25,000/month) as Stream 4 and projects it at ~45% of Cycle-4 MRR and ~30% at Year 2 from a small number of accounts. **That projection is currently an assertion, not a validated number.** Needs: (1) 5–10 real named target organisations in Nairobi (NGOs, research institutes, law firms, banks, consultancies) with a plausible route in; (2) confirmation that multi-author — currently excluded until Cycle 5+ — is genuinely required at Cycle 4, since Institutional cannot ship without it; (3) a pricing sanity check against what these organisations actually pay for comms/publishing tooling today; (4) whether SSO is a real blocker or a nice-to-have. | 🔴 blocked | **Scoped 12 Sep 2026.** Flagged plainly: this is the stream that makes the early MRR numbers survivable while the creator base is thin, and it is the one most at risk if Feature 5 slips. If it does not validate, the whole v2.0.0 revenue projection needs revisiting — better to find that out now than at Cycle 4. **REVIEWED 15 Sep 2026, PLAN session (`relay-29`). Answer `BP26091205` before starting this — that row is directly below and names the reason.** **Ordering note, stated rather than fixed by moving the row:** `BP26091205` (the `aquifer` boundary) is this row's blocker, and by the standing rule a blocker should sit above what it blocks. It is left in place because `BP26091205` is 🔴 and an autonomous run skips it anyway, so physically reordering buys nothing and risks more — but if you are picking this up by hand, read that row first, since its answer may change who the target organisations even are. **What this row actually needs, and it is the reason a session cannot simply do it:** items (1) and (3) — naming 5-10 real Nairobi organisations with a plausible route in, and sanity-checking pricing against what they pay for comms tooling today — are market knowledge, not research a session can substitute for. Sconl has it; the backlog does not. **Items (2) and (4) are different and a session CAN answer them:** whether multi-author is genuinely required at Cycle 4 (currently excluded until Cycle 5+, and Institutional cannot ship without it) and whether SSO is a real blocker are answerable from the canon and the feature set. **Worth splitting on that line when this is picked up** rather than treating the row as uniformly blocked on Sconl. **The stakes, restated because they justify doing this early:** this is the stream that makes early MRR survivable while the creator base is thin, and it is the one most at risk if Feature 5 slips. **UNBLOCKED 15 Sep 2026 — `BP26091205` resolved, per Sconl: *"keep it separate."*** aquifer is its own venture and is not this tier under another name, so the target organisations are Press's own to identify and nothing about this row's framing changes. **It can start now, and it should start with the half a session can actually do — that split is the useful thing this unblock produces.** **Items (2) and (4) are answerable today from the canon and the feature set, by any session, needing nothing from Sconl:** whether multi-author is genuinely required at Cycle 4 — it is currently excluded until Cycle 5+, and Institutional cannot ship without it, so that is a real sequencing conflict to resolve on paper — and whether SSO is a hard blocker or a nice-to-have for the organisations in question. **Items (1) and (3) are market knowledge and cannot be substituted for by any amount of desk work:** naming 5-10 real Nairobi organisations with a plausible route in, and sanity-checking pricing against what those organisations actually pay for comms and publishing tooling today. **Do (2) and (4) first and independently of the rest.** If multi-author turns out to be a genuine Cycle-4 blocker, that changes when the tier can ship at all — and it is discoverable without leaving the repo, so finding it out before anyone invests in a target list is strictly cheaper. |
| BP26091501 (new, 15 Sep 2026 — carried over from iSconl's `PA26091101`, which closed because the method and everything it operates on now live here) | **Catalogue and prioritise the next document archetypes, using the three-axis method proven on `weekly-status-brief`: skeleton × lens × domain schema** | **A catalog-and-prioritise pass, not a single build — deciding WHICH document types matter next and in what order is itself the work.** **The method is already proven** and is what makes this cheap: **skeleton** (the fixed sections a document type always has) × **lens** (who reads it, which drives register and label vocabulary) × **domain schema** (what kind of work fills it). That is how `weekly-status-brief` was taken from idea to buildable, and it should be applied rather than reinvented per type. **Candidates already identified, carried over rather than rediscovered:** (1) **`page-truth-brief`** — already built, and could retroactively gain the lens and label-pool layer if a non-internal audience ever reads it; (2) a **proposal/report archetype** and (3) a **meeting-notes archetype**, both named as placeholders in iSconl's `scope/docs/document-generation-canon.md` §5 naming-profile table and never specified; (4) the **Qpress Weekly Brief newsletter**, which is a genuinely different archetype family — an SEO post plus newsletter rather than an internal alignment brief — and shares the method while sharing nothing else. **What this row must produce to be finished:** a ranked list with a one-line justification per entry, and for the top one or two, enough skeleton/lens/schema detail to mint a real `build.md` row. **Do not scope all four.** | ⬜ ready | **Created here 15 Sep 2026, PLAN session (`relay-29`), closing iSconl's `PA26091101`.** **Why it moved rather than closed outright:** the three-axis method, the archetype system, the generator engine and every template moved to Qpress (`BP26091201`, canon Feature 5, D-009). Running a "which archetype next" pass inside iSconl would have prioritised work iSconl cannot do, against a backlog that is not iSconl's. Its own text already half-knew this, noting the Qpress newsletter design was "living in `qspace-press`, not here, but sharing the same skeleton/lens/schema method." **Deliberately not re-minted into an iSconl ID** — minting a row in the project work has left is the orphaning problem in reverse. **Sequencing, and it matters more than the row's size suggests:** this should run **after** `BB26091208` ships the `weekly-status-brief` archetype, not before. The method is proven on paper but has not yet survived one end-to-end build inside Qpress, and prioritising three more archetypes against a method that has not been executed once here risks committing to a shape that the first real build changes. **One question worth answering as part of the pass rather than assuming:** whether `page-truth-brief` retroactively gaining a lens layer is genuinely wanted, or whether it is only a candidate because it happens to exist. |
<!-- BP26091401 CLOSED 15 Sep 2026 — its central premise is reversed by Sconl's
     own decision the same day: "it stays separate, ispark is its own
     product."

     This row's title is "iSpark is cancelled — the need it existed for folds
     into Qpress instead." The fold does not happen. iSpark stays its own
     product, so there is no Qpress design question here to answer, and the
     row's entire remaining scope — how a tenant's course content gets served
     inside Qpress — is moot.

     WHAT STILL STANDS, and must not be lost with the close: the 14 Sep
     cancellation of iSpark as a DEDICATED MULTI-TENANT FLUTTER APP. Sconl
     cancelled that scope and has not revived it. "Its own product" and "not a
     dedicated mobile app" are compatible — a product need not be an app — so
     nothing here brings the Flutter build back. BL26082601 stays closed as
     won't-build on its original basis.

     WHAT IS GENUINELY STILL OPEN, carried to iSconl's PL26091501 rather than
     left here: if iSpark is its own product and is not a dedicated mobile
     app, what is its delivery surface? That is the one question the
     positioning answer does not settle, and it belongs with the iScroll row
     which has to act on it.

     NOTHING IS LOST. The repo github.com/isconl/ispark stays untouched on
     main with its 5 commits, and the real completed work remains reusable:
     the canonical server-side markdown parser (spark/lib/learning-parser.js,
     POST /learning/parse-md) with mobile inline rendering parity for
     equations, charts, maps and images; a Library selection UI; release
     signing config; and the hub-ispark API gateway pattern in
     lib/api/client.dart. That was always this row's own position and it is
     unchanged by the reversal.

     The three-way Qpress/Aria/iSpark positioning question this row's addendum
     widened into is now answered at iSconl's PL26091406, which closed today.

     Original row text archived below.

<!-- BP26091201 CLOSED 15 Sep 2026, PLAN session (relay-29). This row is a
     decision record, and its own Status cell already read "Decided, canon
     written -- open questions tracked in BP26091202-BP26091205". A decision
     that is made, written to canon and whose follow-ups are tracked elsewhere
     is finished work sitting in an execution queue, so it moves out per
     STANDING-RULES rule 1.

     Its content is not lost and does not depend on this row: it landed 12 Sep
     2026 in work/_arc/qspace-press/canon-canvas/20260912_..._v2_0_0.md as
     Feature 5 (Document Generation & Archetypes), D-009, and the thin-client
     contract. The canon is the durable record; this row was the tracking
     wrapper around it.

     Confirmed still live and therefore genuinely carrying the open questions:
     BP26091204 (Institutional tier) and BP26091205 (aquifer boundary) remain
     in plan.md. BP26091202 and BP26091203 are closed in this same pass,
     because both are likewise resolved.

     Worth recording for anyone tracing the iSconl side: this row's freeze of
     six iSconl rows (BA26091101-BA26091105 plus BI26091202) resolved on 15
     Sep. PA26091202 confirmed that four of the five brief rows map onto
     existing Qpress rows, the fifth (Doc ID generation) is folded into
     BB26091208, and BI26091202's accent-colour plumbing became a row in this
     project. Nothing from that freeze was dropped.

     Original row text archived below.

<!-- BP26091202 CLOSED 15 Sep 2026, PLAN session (relay-29). Its own Status cell
     already read "RESOLVED 12 Sep 2026 by Sconl -- archetype, with AI filling
     the individual fields." A question he has answered is not a decision
     queue item.

     The decision: the weekly brief becomes a deterministic archetype on the
     node-tree path, and AI fills individual fields on request rather than
     authoring the document. Recorded as canon D-013. Sconl's words: "let us
     go with the archetype and the ai still filling the individual fields."

     The rule it established is the part that outlives this row, and it lives
     in canon rather than here: AI drafts one field, never a whole doc --
     inherited verbatim from iSconl's generation canon and now governing all
     of Feature 5, not just this archetype.

     Both consequences confirmed still tracked before closing, so nothing
     falls through: BB26091204 is unblocked and live in build.md, and the
     existing Friday auto-draft setInterval in scope/lib/status-brief.js must
     be rebuilt as a scheduled job invoking the archetype rather than
     inherited as a second code path -- that is BB26091208, also live. The
     existing AI-drafted email pipeline may survive as a consumer of the
     archetype, never as a parallel implementation.

     Original row text archived below.

<!-- BP26091203 CLOSED 15 Sep 2026, PLAN session (relay-29). Status already read
     "Resolved into canon D-012."

     The finding: iSconl's document-generation-canon section 3.2 describes a
     nested signal / substance.{highlights,decisions,risks_blockers} /
     trajectory.{...} shape. The implementation stores three flat
     JSON-encoded string arrays -- SIGNAL, SUBSTANCE, TRAJECTORY -- one per
     TSV cell. Designing against 3.2 would be designing against a document
     rather than against data, with an unwritten migration hidden underneath
     and no consumer currently wanting it. Qpress models the flat shape and
     adds risks_blockers, anticipated Q&A and next_brief_date as first-class,
     since it is defining the model fresh.

     The row asked to be "kept here as a standing warning", and that is the
     one thing worth arguing with, so it is stated rather than quietly
     overridden: a standing warning does not belong in an execution queue. It
     belongs where someone about to make the mistake will actually read it,
     which is the canon -- and it is already there as D-012. Keeping a
     resolved row in plan.md as a reminder makes the queue a worse instrument
     without making the warning more visible.

     The warning's substance is worth restating once here because it
     generalises past this instance: the canon doc was the more
     authoritative-LOOKING artefact and it was the one that was wrong. It was
     caught by reading the source directly rather than trusting the document.
     That is the same failure class as a record that looks current because
     nothing checked it.

     Original row text archived below.

<!-- BP26091205 RESOLVED 15 Sep 2026, per Sconl: "keep it separate."

     THE DECISION: aquifer stays its own venture. It is NOT Qpress's
     Institutional tier under another name, and neither absorbs the other.
     This unblocks BP26091204 (scope the Institutional tier), which was
     waiting on this answer because it could have changed who the target
     organisations even are.

     THE REASONING, recorded because it is what stops this being
     re-litigated a third time. aquifer is a registered venture (ven-aquifer,
     Phase 0 from 28 Jul 2026) with deliberate clean-room IP separation, and
     it is connected to wellspring rather than to Press. The two solve
     different problems: aquifer is content operations across a fleet of
     near-identical websites — keeping content correct and consistent after a
     site exists — while Press's Institutional tier is multi-author
     organisational publishing plus structured document generation. Adjacent
     customers, different products.

     WHY IT KEPT RESURFACING, and why that was not anyone misreading it. The
     same adjacency surfaced independently three times in two projects on two
     days: here; in aquifer's own BP26091102, which claimed to resolve this
     row; and in iSconl's PL26091501, which floated reframing iScroll as an
     Aquifer or Institutional-tier customer. Three independent surfacings is
     a signal that the boundary was genuinely unclear, not that three sessions
     each misread it. That is the case for asking rather than inferring.

     ONE CORRECTION CARRIED INTO aquifer's BP26091102 in the same pass:
     that row claimed to have already resolved this question. It resolved a
     DIFFERENT boundary — aquifer against QSpace PAGES, site-building versus
     ongoing content ops — which its argument genuinely does settle. It did
     not settle aquifer against Press's Institutional tier, which is what this
     row asked. Both boundaries are now closed, but by two different answers,
     and the row is corrected so nobody reads one as having covered both.

     Original row text archived below.

<!-- BP26091206 RE-MINTED BW26091501 and moved to work.md, 15 Sep 2026, per
     Sconl: option (b), sequence-only cycles — plus indicative dates. Both,
     not either. His addition, in his own words: "feel free to readjust the
     dates, i want a close to accurate picture."

     It is now a canon rewrite rather than a decision. Filed as WORK rather
     than BUILD because it is a document rewrite, not a code or feature build.

     WHY "BOTH" IS COHERENT RATHER THAN A HEDGE, since it can read as one: the
     sequence is what the project commits to and is judged against; the dates
     are a planning aid that can move without anything being broken.
     Separating them is exactly what stops a slipped date from discrediting
     the whole plan, which is what happened to canon v1.0.0 — it promised a
     first paying subscriber by 2 June 2026, which passed over three months
     ago with Pre-Cycle 0 still in progress.

     The dates must be RE-DERIVED from where the project actually is, not
     carried forward, and they must be labelled in a way that survives being
     skim-read — a reader who sees a date treats it as a promise unless the
     page makes that impossible.

     THE PART THAT IS NOT OPTIONAL UNDER EITHER OPTION, and was flagged in the
     original row: Feature 5 (Document Generation and Archetypes) is currently
     absent from the Cycle Breakdown, the Quality Gates, the Risk Register and
     the Appendices. A whole feature missing from the risk and quality
     surfaces of the governing document is not a labelling problem.

     One fact that changed since this was raised and makes the re-baseline
     worth anchoring rather than guessing: BB26091203, the generation-engine
     rewrite, is genuinely in progress with checkpoint 1 landed 14 Sep 2026
     (q-space/press dev 2ee9d95). There is real throughput to reason from now
     rather than a standing start.

     Original row text archived below.

<!-- BP26091301 CLOSED 15 Sep 2026, PLAN session (relay-29) — folded into the two
     rows it existed to inform, exactly as its own Notes instructed.

     That row said so itself: "Filed as PLAN rather than directly amending
     build.md because it's new information those two already-queued rows need
     folded in before they're picked up, not a row of its own work; whoever
     picks up BB26091204/BB26091208 should read this row's fix pattern before
     writing the renderer, then this row can close." This is that fold, and
     that close.

     INTO BB26091204 (Creator Studio wizard + live preview): the verified A4
     print pattern -- margins from @page never from container padding; a plain
     block with min-height only, no fixed height, no aspect-ratio, no
     overflow-y; break-inside:avoid on rows and list items -- plus both traps
     and, critically, why each one LOSES content rather than misplacing it. A
     flex-column page container makes Chromium's print engine drop content
     outright across a page break. The aspect-ratio + overflow-y "simulate a
     page on screen" approach is worse, because a print engine clips a
     scrolling container to its fixed box height rather than paginating it, so
     everything past the fold vanishes from the PDF. Sconl's WYSIWYG
     requirement is recorded there as a standing rule rather than a
     preference, and min-height is noted as the primitive that makes it true:
     an on-screen overflow becomes an honest signal that the PDF will spill
     too.

     INTO BB26091208 (weekly-status-brief archetype): the CDP finding --
     Chrome's --print-to-pdf-no-header switch no longer exists (confirmed
     absent from the binary's strings table, Chrome 151), so the renderer must
     drive Page.printToPDF over the DevTools Protocol with displayHeaderFooter
     false, not a bare CLI invocation. Also the two authoring-side rules that
     belong in the archetype's field guidance and AI-assist prompt rather than
     being applied once by hand: three items maximum per bulleted section, and
     no em dashes or other AI-tell phrasing.

     Nothing here was a decision needing Sconl. The pattern was root-caused,
     fixed and verified in session on 13 Sep via headless Chrome and pdfinfo,
     with both the broken and fixed states reproduced, and the reference file
     work/dev/Systems/iSconl/scope/docs/examples/weekly-status-brief.reference.html
     was repaired in place so the two build rows inherit the fix rather than
     the bug.

     Original row text archived below.

<!-- BP26090802 resolved 9 Sep 2026, PLAN session: all 3 sub-questions
     answered. (1) Repo reality: q-space/press's existing single commit
     (312a999, 6 May 2026) is a bare, untouched Flutter scaffold, per
     Sconl explicitly for the future mobile companion app, not the web
     platform — genuinely intentional, not stale cruft. (2) Stack
     reconfirmed as-is, per Sconl: D-001 through D-007's reasoning holds,
     no re-litigation needed. (3) Sconl then corrected this session's own
     assumption that the web platform needed a SEPARATE new repo — "the
     web platform is the one that should use the press repo... is there
     a way to have both scaffolding under one repo?" Yes: the canon
     doc's own apps/web + apps/api file-structure diagram already
     describes exactly this monorepo shape; apps/mobile for Flutter is
     the same pattern extended. Week 1 checklist turned into concrete
     rows: BO26090901 (organize.md, repo restructure — do first),
     BB26090901/BB26090902 (build.md, the pure-code portion). The
     external-accounts portion of Week 1 re-minted below as
     PP26090901, since it's blocked purely on Sconl's own hands, not
     something a build session can execute. -->

<!-- PP26090901 narrowed 9 Sep 2026, PLAN session: Supabase and Upstash
     REMOVED from this list entirely, per Sconl's OCI-self-hosting
     direction (see BB26090903/BB26090904, build.md) — self-hosted
     Postgres/Redis containers replace both, zero external account
     needed for either. 8 signups down to 6. -->

<!-- PP26090902 and PP26090901 both MOVED to the new hands.md 12 Sep 2026,
     PLAN session, per Sconl. Neither was a scoping gap: one is a domain
     purchase needing a funded Cloudflare account, the other is four
     external signups needing identity/payment details a session does not
     have. Re-minted BH26091202 and BH26091201 respectively — note the
     re-mint also FIXES a malformed ID: this project's STANDING-RULES
     specifies `B<letter><YYMMDD><NN>`, so `PP...` was never a valid ID
     here (it follows iSconl's newer two-letter category+domain scheme,
     which this project has not adopted). See organize.md for the row
     tracking that drift properly. Original text archived below.

     PP26090902 (new) | **Register `qspacepress.com` — $10.46/year via Cloudflare Registrar (at-cost, no markup), same account as `acexoft.com` — deliberately deferred, needs Sconl's Cloudflare account funded/billing loaded first** | **Once ready: Cloudflare dashboard → Domains → Registrations → Buy domain → search `qspacepress.com` (confirmed available, 9 Sep 2026 live check) → complete the purchase. Registering it in the same Cloudflare account as `acexoft.com` means it's immediately usable as a DNS zone too — no separate "add to Cloudflare" step needed afterward, registration and zone setup happen together.** | 🔴 Deferred, per Sconl — his own account/billing action | **Found 9 Sep 2026, PLAN session, mid-`PP26090901`'s Resend domain-verification step:** `mail.qspacepress.com` can't be DNS-verified because `qspacepress.com` itself doesn't exist anywhere yet — not registered by Sconl on any registrar. Checked live: available, $10.46/year, "Top pick." **Explicitly deferred, per Sconl** ("can we defer it? i havent loaded the account") — not a scoping gap, just sequencing: he wants his Cloudflare account funded before this purchase, not right now. **No longer blocks `PP26090901`** — Sconl redirected that row to verify `mail.acexoft.com` instead (already-owned domain, unblocks immediately, generalizes across every acexoft-brand service), so Resend's domain verification proceeded without waiting on this purchase. **Still blocks:** the real production hosting target (`main` branch, per the canon doc's own `https://qspacepress.com` target) and adding `mail.qspacepress.com` as Resend's brand-facing production sender later. Does NOT block `BB26090904`'s staging work, which lives entirely under the already-owned `qpress.acexoft.com`, a different domain. **Revisit this row once Sconl confirms the Cloudflare account is funded.** |
