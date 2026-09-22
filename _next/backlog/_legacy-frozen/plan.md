# P — Plan

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

     | BP26091407 (new) | **`app/mobile`'s scope should be corrected: Flutter targets mobile AND desktop, not mobile-only — don't let desktop get silently dropped** | **Per Sconl: "we would not just have mobile, we would have even desktop ones, ensure this is not overlooked or forgotten."** Confirmed live on OneDrive this session: the existing `app/mobile` Flutter scaffold already carries `android/ios/linux/macos/windows/web` platform folders — desktop targets already exist structurally, just never mentioned in this project's own scoping language (`app/mobile`'s own name undersells what it is). **Next session that touches this scaffold should:** (1) confirm all three desktop targets (Windows/macOS/Linux) actually build, not just the mobile ones; (2) consider whether `app/mobile` should be renamed to something that doesn't imply mobile-only (e.g. `app/companion` or `app/desktop-mobile`) — naming call for whoever picks this up, not decided here; (3) fold this into whatever UI/UX work the companion app gets, so a desktop layout isn't an afterthought bolted onto a phone-shaped design. Same requirement applies to `qspace-pages`'s own forthcoming Flutter companion app (`BB26091401`, that project's `build.md`) — not just Press. | ⬜ Queued — no urgency, just don't let it get forgotten | **Scoped 14 Sep 2026, PLAN session, per Sconl's direct reminder.** **REVIEWED 15 Sep 2026, PLAN session (`relay-29`) — correctly filed, but it is a constraint on future work rather than work itself, and that is worth making explicit so nobody waits for it to become actionable.** Nothing here can be executed yet: item (1), confirming the Windows, macOS and Linux targets actually build, needs a scaffold that compiles, and Qpress's own engine is still mid-rewrite. **The real risk this row names is not that the desktop targets are missing — they already exist structurally — it is that `app/mobile`'s name teaches every future session to think mobile-only, and a phone-shaped design gets built before anyone notices.** That risk is live now, not when the row becomes actionable. **Recommendation, since it costs nothing and closes the risk permanently: fold the desktop requirement into the project's canon rather than leaving it as a backlog row to remember.** A constraint in the canon is read before substantive work by `CLAUDE.md` §15; a constraint in `plan.md` is read only by whoever happens to walk the queue. The rename in item (2) can then be decided by whoever builds the scaffold, with the requirement already binding either way. | -->
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

     | BP26091403 (new) | **"Canvas" — rename the course-module publish button to a live-editable HTML artifact, and share courses with the Viva team through it. This belongs to Qpress, not Aria — confirmed by reading Aria's actual repo.** | **Sconl's ask: change the (publish artifact) button to "Canvas," a live editable HTML artifact, and use it to share Academia/Grove course modules with his Viva team. He also asked whether this fits Qpress or Aria better, since Aria is queued as a separate standalone product.** **Investigated `github.com/Sconl/aria` directly this session (README, POSITIONING.md, file tree) — Aria is the wrong home.** Aria (Aria Course Engine / ACE) produces **narrated video courses**: its pipeline output is `.mp4` + `.pptx` + `.mp3` + a cue-sheet PDF, built for corporate-academy/L&D video production at volume, white-label per customer. It has no HTML/web-document output anywhere in its stack — a completely different medium from "a live editable HTML artifact." Building Canvas inside Aria would mean bolting an unrelated web-document editor onto a video-production pipeline. **Qpress is the right home** — it already owns exactly this shape: an HTML-based Creator Studio, a live-editable document/publishing target (`BB26091204`'s weekly-brief wizard + live preview, `BP26091301`'s print-correct A4 HTML template work), and the thin-client relationship with iSconl (`BP26091201`) that already routes iSconl's document generation through Qpress's own editor. **Concretely:** rename the existing "Publish artifact" action to **Canvas** in Qpress's Creator Studio, and give it a course-module content type alongside whatever document types `BB26091204`'s archetype system already handles — a course module becomes one more thing Canvas can render/edit/share, not a special case. **Sharing with Viva:** reuses the same public/unauthenticated sharing mechanism iSconl's own `BL26091102` already built for standalone module export (`/api/public/learn/:course/:slug` pattern) — Qpress's Canvas becomes the *editable* front end, iSconl's existing export becomes the *read-only* fallback until Canvas subsumes it. | 🔴 **Blocked on `PL26091406`** (iSconl `plan.md`, Tier 1) — feeds off `BB26091204`; `BP26091301` closed 15 Sep, folded into `BB26091204`/`BB26091208` | **Scoped 14 Sep 2026, PLAN session, per Sconl's brainstorm, "this cycle my theme is leverage, i need to make this tool a product."** Aria stays a fully separate, standalone product — see `aria`'s own `plan.md` row written this same session for its queue position and domain reservation (`aria.acexoft.com`). Nothing in this row touches Aria's build. **RE-FLAGGED 15 Sep 2026, PLAN session (`relay-29`) — this row was marked "Queued, no new infra needed", and that reads as further along than it is. It is blocked, and on the same question as `BP26091401` directly below it.** **The hidden assumption:** this row's central move is to give Canvas "a course-module content type alongside whatever document types the archetype system already handles — a course module becomes one more thing Canvas can render/edit/share." **That is a decision that Qpress owns course-module rendering. It is precisely the question `PL26091406` is paused on, and that pause is Sconl's.** On 14 Sep he said, of folding iSpark into Qpress: *"considering the aria open question, let it hold on for us to brainstorm the best positioning for each product for the best market value."* **`BP26091401` correctly records that pause. This row does not, and would quietly settle it by building it** — which is how a paused decision gets made by implementation rather than by choice. **The Aria half of this row is unaffected and stays settled.** The investigation that ruled Aria out was done by reading its actual repo, not inferred: Aria's pipeline output is `.mp4` + `.pptx` + `.mp3` + a cue-sheet PDF, with no HTML or web-document output anywhere in its stack. Canvas is a live-editable HTML artifact. That is a different medium, and nothing in the positioning brainstorm is likely to change it. **So what is blocked is narrower than the whole row: the rename of "Publish artifact" to Canvas, and Canvas as a live-editable HTML surface, are Qpress work under any outcome. It is specifically the course-module content type and the Viva course-sharing use case that wait on `PL26091406`.** **If the brainstorm lands and Qpress does own course content, this row is immediately buildable as written and nothing here needs rescoping** — the design work is sound, it is the premise that is unconfirmed. | -->
<!-- BP26091402 resolved 14 Sep 2026, PLAN session, per Sconl: option (a) --
     native Rust rewrite of the generation engine, matching BUILD's own
     recommendation (the only option consistent with canon D-002's reason
     for choosing Rust at all). Re-minted BB26091203 (was already that ID,
     re-scoped in place from "port" to "rewrite"), moved back to build.md,
     unblocked. Original text archived below.

     BP26091402 (new) 🔴 | **DECIDE: how does `BB26091203`'s "port" actually reach `app/api` — a real Rust rewrite of `render-docx`/`render-pdf`, or a JS runtime/sidecar embedded in a Rust service?** | **Bounced back from `build.md`'s `BB26091203` before any code was written — this is an unscoped architectural call, not a translation task, and the row's own premise turns out to be wrong once the target runtime is actually checked.** `BB26091203`/`BP26091201` both describe this as a "port," reasoning "the engine is already correctly factored, which is why this is a port rather than a rewrite" — true for JS→JS reuse, **not checked against what `app/api` actually is.** Confirmed live: `app/api/Cargo.toml` is a plain Axum/SeaORM Rust binary with **zero** JS-runtime crate (no `deno_core`, `boa`, `rquickjs`, `v8`, nothing) and zero document-generation crate of any kind. The two renderers that matter most, `render-docx.js` (4.9KB) and `render-pdf.js` (4.0KB), are thin wrappers around the mature JS-ecosystem libraries `docx` and `pdfkit` — there is no drop-in Rust equivalent already vetted anywhere in this codebase or canon. **The real options, genuinely different engineering efforts:** (a) hand-rewrite the ~34KB of engine/renderer/archetype JS in Rust against `docx-rs`/`printpdf`-class crates, accepting a real fidelity risk (every archetype's rendered `.docx`/`.pdf` needs re-validation against the JS output, byte-identical generation is explicitly required by `BB26091208`'s acceptance criteria) — the honest "port" in spirit, but a first-time undertaking with no existing pattern to follow in this fleet; (b) run the existing Node engine as a sidecar/microservice the Rust API calls internally, preserving the JS code and its exact output completely unchanged — fast and low-risk, but directly cuts against canon D-002's own stated reasons for choosing Rust in the first place ("binary deployment — no runtime dependency management"), and there is no existing precedent anywhere in the fleet (which is otherwise all-Node) or in Qpress's own canon for a mixed-runtime service; (c) embed a JS engine (e.g. `rquickjs`/`boa`) inside the Rust process itself — keeps deployment single-binary, but the JS `docx`/`pdfkit` libraries were never written against an embedded-engine environment (they assume Node's `fs`/`Buffer`/streams) and would need real compatibility work, unproven. | 🔴 Blocked on Sconl | **Found 14 Sep 2026, BUILD session, on `BB26091203`'s own first step** (reading `app/api`'s real Dockerfile/Cargo.toml before committing to an approach, per standing BUILD-session protocol after `BI26091110`'s precedent in iSconl). No code was written against any of the three options — this is a genuine fork in engineering approach with real cost/fidelity tradeoffs only Sconl can weigh, not a scoping gap a BUILD session should guess through. **Recommendation, not a decision:** (a) is the only option consistent with why Rust was chosen for this service at all (D-002) and is the only one that doesn't introduce an unprecedented mixed-runtime or embedded-engine pattern into a codebase that has neither — but it is real, first-time, multi-session work (rewriting DOCX/PDF binary generation from scratch, plus a validation pass per archetype), not a quick follow-on to `BB26091202`. `BB26091203` stays queued in `build.md` pointing here rather than being silently attempted under any one of these three assumptions. -->

| # | Title | Task | Status | Notes |
|---|-------|------|--------|-------|
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

     | BP26091401 (new) | **"iSpark" (dedicated multi-tenant Flutter learning app) is cancelled — the need it existed for folds into Qpress instead; no dedicated mobile app gets built** | **Per Sconl, direct instruction, 14 Sep 2026: kill the standalone Flutter app scope, fold it into Qpress, no dedicated mobile app is needed.** `BL26082601` (iSconl `build.md`) is closed as won't-build on this basis — see that row for the full prior scope (dedicated `vault`+`spark` tenant backend, canonical markdown parser, mobile inline media parity, first tenant "iScroll"). **Nothing is lost:** the repo (`github.com/isconl/ispark`, `main`, 5 real commits) stays as-is, untouched, holding real completed work worth reusing as reference — the canonical markdown parser + mobile inline image/chart/map/equation rendering (rendering-engine parity already shipped, recorded in iSconl's `done.md`), and a `hub-ispark` API gateway pattern in `lib/api/client.dart`/`main.dart`/`updater.dart` that a BUILD session flagged as possibly worth porting. **Not yet decided, genuinely open — this is the actual scoping work still needed:** iSpark's original purpose was distributing Academia-shaped learning content (structured modules, jargon glossary, progress tracking, per-tenant branding) to an external tenant, not publishing SEO posts/newsletters — a materially different content shape from Qpress's Publishing Engine. "Fold into Qpress" needs a real design pass on how that gets served: does a tenant's course content get authored/published as ordinary Qpress posts/pages (loses the Academia-specific reader chrome — jargon popups, progress bars, five-section fractal structure — unless Qpress's renderer grows those), does Qpress gain a distinct "course" content type alongside posts/newsletters, or does the tenant just get a web-only reader that isn't part of Qpress's post model at all? **Do not build anything against this until that question is answered** — same discipline as `BP26091205`'s aquifer boundary question just below, which this may also intersect with (an external tenant's course content and Aquifer's "content ops for website fleets" pitch could be the same customer shape). | ⬜ Queued — needs a design pass before any code | **Raised 14 Sep 2026, PLAN session, mid-session redirect from Sconl while `relay-6b` (BUILD) was already mid-`BL26082601` — BUILD was stopped before any code was written against the old scope, confirmed read-only exploration only.** **ADDENDUM, same day, a separate concurrent PLAN session with Sconl:** he referenced this exact instruction ("i asked that ispark be folded into qpress") and then went further than this row currently states — **"considering the aria open question, let it hold on for us to brainstorm the best positioning for each product for the best market value."** Read together, the two sessions are sequential, not contradictory: this row's "cancelled, folds into Qpress, only the HOW is open" was the first instruction; the newer one widens the pause to include WHETHER Qpress is even the right destination, pending a three-way Qpress/Aria/iSpark positioning conversation. **Do not treat "folds into Qpress" as settled** until that conversation happens — treat this row's central design question (how a tenant's course content gets served) as itself gated on that broader positioning call, not just on someone picking one of the three options already listed. Tracked in parallel at `iSconl`'s own `plan.md` (`PL26091406`) and `aria`'s own `plan.md` (`BP26091401`, that project's own ID namespace). **CONFIRMED BLOCKED 15 Sep 2026, PLAN session (`relay-29`), and `PL26091406` has been re-tiered to Tier 1 in iSconl on the strength of this row plus one other.** This row's addendum is correct and should be read as the operative instruction rather than its title: the cancellation of the standalone Flutter app stands, but *"folds into Qpress"* does not, and this row's central design question is gated on the three-way positioning brainstorm rather than on someone picking one of its three listed options. **One correction to a claim made elsewhere that bears on that brainstorm: Qpress is no longer stalled.** iSconl's `PL26091501` argues against folding into Qpress partly because Qpress is *"still pre-Cycle-0 with its own generation engine mid-rewrite, currently hard-blocked on a machine Application Control policy."* **That is out of date — `BB26091203` is 🟡 in progress, checkpoint 1 landed 14 Sep 2026 on `q-space/press` `dev` `2ee9d95`.** The blocker cleared. **That materially strengthens the fold-into-Qpress option**, which was partly penalised for depending on something itself blocked, and Sconl should have it when he holds the brainstorm. | -->
| BP26090801 (new) | **Rename the isconl agent's "Writer" space to "Press"/"QPress" and rebuild it as a thin client into Qpress's own Publishing Engine API — blocked until that API exists** | **Not build-ready yet.** Qpress has no repo, no API, no Feature 1 (Publishing Engine) shipped — canon doc status is "Pre-Cycle 0, no commits yet." Writer today generates internal business documents (reports/briefs, archetype/wizard-driven, tied to `scope/tasks.tsv` engagements/projects, output `.docx`/`.md` via `scope/lib/generate/`) — a genuinely different shape from Qpress's Publishing Engine (SEO-indexed public posts + newsletter delivery to external subscribers, Tiptap editor). Rebuilding Writer as a thin client requires a real target API to point it at. **Do not attempt before `Feature 1: Publishing Engine` (Qpress's own Cycle 1 milestone) ships and exposes a usable API.** | ⬜ Blocked on Qpress's own Cycle 1 | **Scoped 8 Sep 2026, iSconl PLAN session, per Sconl's explicit decision** (accepted despite the investigation finding real shape differences between Writer and Qpress's Publishing Engine — his call to proceed anyway, not re-litigated here). Cross-referenced from `PL26090801` in `Systems/iSconl/_next/backlog/plan.md` (now closed there — the 3 open questions it asked are resolved; this row and `BP26090802` below are the concrete follow-through). **Revisit this row once Qpress's Cycle 1 actually ships** — re-check at that point whether Writer's current archetype/wizard model can realistically wrap Qpress's post-editor API, or whether "thin client" needs to mean something narrower (e.g. Writer keeps its own document-generation archetypes for internal docs, and only a NEW archetype type is added that posts to Qpress) — not assumed here, a real design question once there's an API to design against. **UPDATED 12 Sep 2026, PLAN session:** premise substantially overtaken by Sconl's direction that ALL builds happen inside Qpress with iSconl as a thin client — see `BP26091201` below, which supersedes this row's framing. Also corrected: the investigation's "Writer has a Tiptap editor" premise was wrong in both directions. iSconl has **no** rich-text editor at all (no Tiptap/ProseMirror/Quill/Lexical/contenteditable anywhere in `hub`) — it has a 3-step wizard rendering an HTML form from archetype field definitions. And `hub` holds **no** generation business logic: it is already a pure HTTP client of `scope`+`spark` via 13 route mappings in `hub/lib/api-compat.js`. The "thin client" is therefore re-pointing 13 routes, not a rebuild. **REVIEWED 15 Sep 2026, PLAN session (`relay-29`) — correctly blocked, nothing to change, and nothing needed from Sconl.** The blocker is real and unchanged: Qpress's Feature 1 Publishing Engine has not shipped, so there is no API to point a thin client at. **Worth carrying forward, because this row has been partially superseded twice and the layers are easy to misread:** its original framing (rebuild Writer against Qpress's editor API) was overtaken on 12 Sep by `BP26091201`, which made iSconl a thin client of Qpress wholesale; and its own investigation premise was corrected in both directions — iSconl has **no** rich-text editor at all, and `hub` holds **no** generation logic, being already a pure HTTP client of `scope` and `spark` via 13 route mappings in `hub/lib/api-compat.js`. **So the eventual work is genuinely small**, and the row's own "revisit once Cycle 1 ships" instruction stands: re-check then whether Writer's archetype model can wrap Qpress's post-editor API, or whether thin client needs to mean something narrower. **Related and now tracked: `BP26091205` in `qspace-press` `build.md` already carries the concrete re-pointing of those 13 routes**, so this row is the design question and that row is the execution. |
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

     | BP26091201 (new) | **Design decision record — Qpress absorbs the editor, generator engine, archetypes and templates; iSconl's qpress space becomes a thin client. ALREADY EXECUTED into canon v2.0.0; this row tracks the remaining open questions only.** | **Landed 12 Sep 2026 in `work/_arc/qspace-press/canon-canvas/20260912_..._v2_0_0.md`** as Feature 5 (Document Generation & Archetypes), D-009, and the thin-client contract. **What ports:** `doc-builder`, `registry`, `node-tree`, `naming`, `style`, all 3 renderers, `output.js`, all 10 archetype modules. **What stays in iSconl and is passed over the API:** `generated_docs.tsv` indexing, OneDrive push/browse, `tasks.tsv` binding + `DELIVERABLE` write-back, engagement/venture target resolution. **API boundary is "gathered activity," not data access** — Qpress never learns what an `ORG_ID` or `SOURCE_REF` is. | 🟢 Decided, canon written — open questions tracked in `BP26091202`–`BP26091205` | **Scoped and written 12 Sep 2026, PLAN session, per Sconl's direct instruction.** Six iSconl rows frozen as a result (`BA26091101`–`BA26091105` + `BI26091202`) — relay-4f holds them, logged with pointers here rather than silently skipped; no code was written before the freeze landed, so nothing was lost. | -->
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

     | BP26091202 (new) 🔴 | **DECIDE: does the weekly brief become a deterministic Qpress archetype, or stay an AI-drafted email pipeline?** | **Two different things currently share one name.** (a) `weekly-status-brief` **as an archetype does not exist** — specified in iSconl's `scope/docs/document-generation-canon.md` §3.2, never implemented. (b) What DOES exist is `scope/lib/status-brief.js`: reads `active_subjects.tsv`/`status_briefs.tsv`/`tasks.tsv`/`circle/interactions.tsv`, calls spark to **AI-draft**, emails via vault Graph, auto-drafts every Friday on a timer, and bypasses `lib/generate/` entirely. **Recommendation: make it a proper archetype on the node-tree path.** It is what the canon always intended, it gains HTML output and therefore publishability, generation stays deterministic and reproducible, and AI can still fill individual fields the way `spark`'s existing `research-field`/`full-draft` helpers already do. The AI-drafted email pipeline can continue to exist as a *consumer* of the archetype rather than a parallel implementation. | ✅ **RESOLVED 12 Sep 2026 by Sconl — archetype, with AI filling individual fields** | **Decision recorded as canon D-013.** Sconl's words: *"let us go with the archetype and the ai still filling the individual fields."* The brief becomes a deterministic archetype on the node-tree path; AI fills **individual fields** on request and never authors the document — the rule **"AI drafts one field, never a whole doc"** is inherited verbatim from iSconl's generation canon and now governs all of Feature 5, not just this archetype. **`BB26091204` is unblocked.** **One consequence carried forward, not silently dropped:** the existing pipeline's Friday auto-draft (`setInterval` in `scope/lib/status-brief.js`) must be rebuilt as a **scheduled job that invokes the archetype**, not inherited as a second code path — tracked as `BB26091208`. The existing AI-drafted email pipeline may survive as a *consumer* of the archetype, never as a parallel implementation. | -->
| BP26091207 (new) | **Lift the `main` branch lock once `qspacepress.com` is registered** | `main` on `q-space/press` was locked read-only 12 Sep 2026 (canon D-014) so nothing can reach a production path that does not exist yet. Protection applied: `lock_branch: true`, `enforce_admins: true`, `allow_force_pushes: false`, `allow_deletions: false`. **To lift:** `gh api -X DELETE repos/q-space/press/branches/main/protection`, or edit it in the GitHub UI under Settings → Branches. **Preconditions before lifting:** (1) `BH26091202` done — domain registered and the DNS zone live; (2) a production deploy target actually exists (tunnel ingress rule + DNS record for `qspacepress.com`, same mechanism as staging); (3) a `deploy-production.yml` workflow written and reviewed — deliberately NOT written yet, since a workflow pointing at a destination that does not resolve is worse than no workflow. | ⬜ Blocked on `BH26091202` (domain purchase — Sconl's hands) | **Created 12 Sep 2026 at Sconl's request.** Note the lock is on the *branch*, not the repo: `dev` and `staging` are entirely unaffected and staging deploys continue normally. The lock is fully reversible by any admin at any time. **BLOCKER VERIFIED STILL LIVE 15 Sep 2026, PLAN session (`relay-29`): `BH26091202` is a live row in this project's `hands.md` — register `qspacepress.com`, about $10.46/year at Cloudflare Registrar, deferred until Sconl's Cloudflare account is funded. Not shipped, not quietly dropped, so this row correctly stays put.** **Nothing here needs a decision from him, only the purchase**, and that is already stated in the right place rather than duplicated here. **Worth saying plainly because a branch lock reads alarming out of context: nothing is broken and nothing is waiting on this.** The lock is on `main` only; `dev` and `staging` are unaffected and staging deploys continue normally. Its whole purpose is to stop anything reaching a production path that does not resolve yet, which is still true. **Do not lift it early to "unblock" something** — the two remaining preconditions beyond the domain (a real production deploy target, and a reviewed `deploy-production.yml`) exist because a workflow pointing at a destination that does not resolve is worse than no workflow. |
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

     | BP26091203 (new) | **Model the brief on the IMPLEMENTED data shape, not iSconl canon §3.2 — recorded as D-012, kept here as a standing warning** | §3.2 describes nested `signal` / `substance.{highlights,decisions,risks_blockers}` / `trajectory.{...}`. The implementation stores three **flat JSON-encoded string arrays** — `SIGNAL`, `SUBSTANCE`, `TRAJECTORY` — one per TSV cell. **Designing against §3.2 means designing against a document rather than against data**, with an unwritten migration hidden underneath and no consumer currently wanting it. Qpress models the flat shape and adds `risks_blockers`, anticipated Q&A and `next_brief_date` as first-class since it is defining the model fresh. | 🟢 Resolved into canon D-012 | **Established 12 Sep 2026 by direct source reading (relay-4f), not inferred.** Recorded because it is exactly the kind of thing that gets rediscovered expensively — the canon doc is the more authoritative-*looking* artefact and is the one that is wrong. | -->
| BP26091204 (new) | **Scope the Institutional tier — the commercial case for Feature 5, and the largest projected stream by Year 2** | Canon v2.0.0 adds Institutional (from KES 25,000/month) as Stream 4 and projects it at ~45% of Cycle-4 MRR and ~30% at Year 2 from a small number of accounts. **That projection is currently an assertion, not a validated number.** Needs: (1) 5–10 real named target organisations in Nairobi (NGOs, research institutes, law firms, banks, consultancies) with a plausible route in; (2) confirmation that multi-author — currently excluded until Cycle 5+ — is genuinely required at Cycle 4, since Institutional cannot ship without it; (3) a pricing sanity check against what these organisations actually pay for comms/publishing tooling today; (4) whether SSO is a real blocker or a nice-to-have. | ⬜ Queued — scoping pass, no code | **Scoped 12 Sep 2026.** Flagged plainly: this is the stream that makes the early MRR numbers survivable while the creator base is thin, and it is the one most at risk if Feature 5 slips. If it does not validate, the whole v2.0.0 revenue projection needs revisiting — better to find that out now than at Cycle 4. **REVIEWED 15 Sep 2026, PLAN session (`relay-29`). Answer `BP26091205` before starting this — that row is directly below and names the reason.** **Ordering note, stated rather than fixed by moving the row:** `BP26091205` (the `aquifer` boundary) is this row's blocker, and by the standing rule a blocker should sit above what it blocks. It is left in place because `BP26091205` is 🔴 and an autonomous run skips it anyway, so physically reordering buys nothing and risks more — but if you are picking this up by hand, read that row first, since its answer may change who the target organisations even are. **What this row actually needs, and it is the reason a session cannot simply do it:** items (1) and (3) — naming 5-10 real Nairobi organisations with a plausible route in, and sanity-checking pricing against what they pay for comms tooling today — are market knowledge, not research a session can substitute for. Sconl has it; the backlog does not. **Items (2) and (4) are different and a session CAN answer them:** whether multi-author is genuinely required at Cycle 4 (currently excluded until Cycle 5+, and Institutional cannot ship without it) and whether SSO is a real blocker are answerable from the canon and the feature set. **Worth splitting on that line when this is picked up** rather than treating the row as uniformly blocked on Sconl. **The stakes, restated because they justify doing this early:** this is the stream that makes early MRR survivable while the creator base is thin, and it is the one most at risk if Feature 5 slips. **UNBLOCKED 15 Sep 2026 — `BP26091205` resolved, per Sconl: *"keep it separate."*** aquifer is its own venture and is not this tier under another name, so the target organisations are Press's own to identify and nothing about this row's framing changes. **It can start now, and it should start with the half a session can actually do — that split is the useful thing this unblock produces.** **Items (2) and (4) are answerable today from the canon and the feature set, by any session, needing nothing from Sconl:** whether multi-author is genuinely required at Cycle 4 — it is currently excluded until Cycle 5+, and Institutional cannot ship without it, so that is a real sequencing conflict to resolve on paper — and whether SSO is a hard blocker or a nice-to-have for the organisations in question. **Items (1) and (3) are market knowledge and cannot be substituted for by any amount of desk work:** naming 5-10 real Nairobi organisations with a plausible route in, and sanity-checking pricing against what those organisations actually pay for comms and publishing tooling today. **Do (2) and (4) first and independently of the rest.** If multi-author turns out to be a genuine Cycle-4 blocker, that changes when the tier can ship at all — and it is discoverable without leaving the repo, so finding it out before anyone invests in a target list is strictly cheaper. |
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

     | BP26091205 (new) | **Resolve the `aquifer` boundary before building the Institutional tier — risk of building the same thing twice** | `aquifer` (registry row, scaffolded 11 Sep 2026) is Sconl's own venture, described as "content ops for website fleets," Phase 0/pre-build, with its own IP/clean-room and building since 28 Jul 2026. Qpress's Institutional tier — multi-author org publishing plus structured document generation — is adjacent enough that the overlap needs naming explicitly. **Questions:** do they share a customer? does one become a channel for the other? is Feature 5 actually the thing aquifer needs, or a competitor to it? See `BP26091102` in aquifer's own `plan.md` for the wellspring connection, which may be the same pattern. | ⬜ Queued 🔴 needs Sconl's framing | **Raised 12 Sep 2026 while writing canon v2.0.0's Stream 4.** Not a blocker for Cycle 1–3; genuinely blocking for Institutional. Worth answering before `BP26091204`'s target-org work, since the answer may change who the targets are. **BLOCKS `BP26091204`, and it is the one to put in front of Sconl first of this project's three open decisions, 15 Sep 2026, PLAN session (`relay-29`).** **Why it ranks above the other two:** `BP26091206` (re-baseline or freeze the timeline) changes how work is described; this one changes whether a whole tier gets built, and by whom. **The question is not really "what is the boundary" — it is one prior question Sconl can answer in a sentence: is `aquifer` a separate venture that happens to look adjacent, or is Qpress's Institutional tier the thing `aquifer` was always going to need?** Everything else follows. If they share a customer, building both is building the same product twice with two brands and two roadmaps. If they do not, the adjacency is cosmetic and `BP26091204` proceeds untouched. **Not a blocker for Cycles 1-3** — genuinely blocking only for Institutional, so it does not hold the current build queue. **One thing worth putting in front of him alongside it, because it may be the same question a third time:** iSconl's `PL26091501` raised, as one of four options for iScroll, reframing it as an Aquifer or Institutional-tier customer rather than a tenant. Sconl has since answered that iScroll is a tenant — **but the fact that the same adjacency surfaced independently in two projects on two different days is itself the signal that the boundary is genuinely unclear**, not that either row misread it. | -->
| BP26091501 (new, 15 Sep 2026 — carried over from iSconl's `PA26091101`, which closed because the method and everything it operates on now live here) | **Catalogue and prioritise the next document archetypes, using the three-axis method proven on `weekly-status-brief`: skeleton × lens × domain schema** | **A catalog-and-prioritise pass, not a single build — deciding WHICH document types matter next and in what order is itself the work.** **The method is already proven** and is what makes this cheap: **skeleton** (the fixed sections a document type always has) × **lens** (who reads it, which drives register and label vocabulary) × **domain schema** (what kind of work fills it). That is how `weekly-status-brief` was taken from idea to buildable, and it should be applied rather than reinvented per type. **Candidates already identified, carried over rather than rediscovered:** (1) **`page-truth-brief`** — already built, and could retroactively gain the lens and label-pool layer if a non-internal audience ever reads it; (2) a **proposal/report archetype** and (3) a **meeting-notes archetype**, both named as placeholders in iSconl's `scope/docs/document-generation-canon.md` §5 naming-profile table and never specified; (4) the **Qpress Weekly Brief newsletter**, which is a genuinely different archetype family — an SEO post plus newsletter rather than an internal alignment brief — and shares the method while sharing nothing else. **What this row must produce to be finished:** a ranked list with a one-line justification per entry, and for the top one or two, enough skeleton/lens/schema detail to mint a real `build.md` row. **Do not scope all four.** | ⬜ Queued — scoping pass, no code | **Created here 15 Sep 2026, PLAN session (`relay-29`), closing iSconl's `PA26091101`.** **Why it moved rather than closed outright:** the three-axis method, the archetype system, the generator engine and every template moved to Qpress (`BP26091201`, canon Feature 5, D-009). Running a "which archetype next" pass inside iSconl would have prioritised work iSconl cannot do, against a backlog that is not iSconl's. Its own text already half-knew this, noting the Qpress newsletter design was "living in `qspace-press`, not here, but sharing the same skeleton/lens/schema method." **Deliberately not re-minted into an iSconl ID** — minting a row in the project work has left is the orphaning problem in reverse. **Sequencing, and it matters more than the row's size suggests:** this should run **after** `BB26091208` ships the `weekly-status-brief` archetype, not before. The method is proven on paper but has not yet survived one end-to-end build inside Qpress, and prioritising three more archetypes against a method that has not been executed once here risks committing to a shape that the first real build changes. **One question worth answering as part of the pass rather than assuming:** whether `page-truth-brief` retroactively gaining a lens layer is genuinely wanted, or whether it is only a candidate because it happens to exist. |
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

     | BP26091206 (new) 🔴 | **DECIDE: re-baseline the cycle timeline, or freeze the numbering?** | Canon v1.0.0 targeted **2 June 2026** for first paying subscriber (Cycle 2 end). That date passed over three months ago with Pre-Cycle 0 still in progress. Canon v2.0.0 corrects every factual claim but **deliberately leaves the Master Timeline and Cycle Breakdown untouched** and flags them stale, because re-baselining is a commitment only Sconl can make. Every cycle's *content* is current and correct; only the dates are wrong. **Two options:** (a) re-baseline the 8 cycles off a chosen start date, absorbing Feature 5 into the sequence; (b) keep cycle numbering as pure sequence with no calendar dates at all, and track velocity instead. | 🔴 Blocked on Sconl | **Raised 12 Sep 2026, PLAN session.** Recommendation is (b) — this project has now missed a calendar target once and dated plans that slip tend to get ignored wholesale rather than corrected. Sequence-only cycles with an explicit "what unblocks the next cycle" per row stay honest. But it is a real choice and it is his. Note Feature 5 is not yet reflected anywhere in the Cycle Breakdown, Quality Gates, Risk Register or Appendices — whichever option is chosen, that pass is needed. **REVIEWED 15 Sep 2026, PLAN session (`relay-29`) — correctly blocked on Sconl, recommendation unchanged and endorsed, going into this session's batched decision list.** **The recommendation stands: option (b), sequence-only cycles with no calendar dates, and an explicit "what unblocks the next cycle" per row.** The reasoning in this row is sound and worth restating in one line because it is the whole argument: this project has already missed a calendar target once, and dated plans that slip tend to get ignored wholesale rather than corrected. **One thing that has changed since this was raised and that makes the choice slightly less costly either way: the project is genuinely moving again.** `BB26091203`, the generation-engine rewrite, is 🟡 in progress with checkpoint 1 landed 14 Sep 2026 (`q-space/press` `dev` `2ee9d95`) — so a re-baseline under option (a) would at least be anchored to real velocity rather than to another estimate. **The part that is NOT optional under either option, and is the reason this cannot sit indefinitely:** Feature 5 is still not reflected anywhere in the Cycle Breakdown, Quality Gates, Risk Register or Appendices. That pass is needed whichever way he decides, so the decision is gating real documentation work, not just a labelling preference. | -->
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

     | BP26091301 (new) | **The `weekly-status-brief` archetype's `html`/`pdf` output needs a real, print-correct A4 template — none exists anywhere yet, and the existing reference file's approach was actively wrong for print** | **Found and fixed 13 Sep 2026, drafting a real weekly brief for Joel Tambaur (Viva Valentia) by hand as a Claude artifact, since no engine exists to generate one.** Confirms `BP26091202`'s framing from the other side: not only did the archetype not exist, nothing that could render one to a correct one-page PDF existed either — checked both `qspace-press` (Feature 1 "Publishing Engine" is an unrelated Tiptap blog/newsletter editor, still unbuilt) and iSconl's own `scope/docs/examples/weekly-status-brief.reference.html` (a static hand-authored demo with a non-functional "Export PDF" button and no `@media print`/`@page` rules at all). **Two real, now-fixed bugs, both root-caused this session and both now patched directly into that reference file so `BB26091204`/`BB26091208` inherit the fix instead of the bug:** (1) a `display:flex; flex-direction:column` page container (used to pin a footer to the bottom of a one-page screen view) makes Chromium's print engine **silently drop content** when the container spans a page break, not just misplace margins — never lay out a print-target page as a paginating flex container; (2) the reference file's original approach (`aspect-ratio:210/297` + `overflow-y:auto`, meant to *simulate* a page on screen) is wrong for print for a different reason: a browser's print engine does not paginate a scrolling container, it clips it to the fixed box height, so content past that point is silently lost in the PDF, not moved to page 2. **The correct pattern, now in the reference file and this row's canonical statement for whoever builds `BB26091204`/`BB26091208`'s renderer:** margins come from `@page { size: A4; margin: <mm> }`, never from the page container's own padding (so an overflow page still gets a correct margin instead of none); the container is a plain block with `min-height` only, no fixed height/aspect-ratio/overflow; `break-inside:avoid` on table rows and list items. **WYSIWYG requirement, explicitly requested by Sconl and now the standing rule for Creator Studio's live preview (`BB26091204`):** the on-screen preview must be structurally identical to the print/PDF output, not a separate approximation — the min-height (not aspect-ratio) approach means an on-screen overflow is an honest signal that the PDF will also spill to a second page, rather than a screen-only illusion of fitting. **Two authoring-side standing rules to fold into the archetype's own field guidance / AI-assist prompt, not just this one document:** cap every bulleted section at 3 items (keeps a one-page brief actually one page); no em dashes or other AI-tell phrasing in generated prose. **Reference implementation, updated in place rather than left to rot:** `work/dev/Systems/iSconl/scope/docs/examples/weekly-status-brief.reference.html` — its one-pager block now prints correctly as a single A4 page (verified via headless Chrome + `pdfinfo`, both before-fix 2-page/content-loss and after-fix 1-page-clean states reproduced and confirmed this session). The actual Joel/Viva brief this was built for is a private Claude artifact, not committed anywhere (real supervisor content, not example content) — the reference file is the durable, generic carrier of the template fix. **Addendum, same day, generating the real Joel PDF: Chrome's CLI `--print-to-pdf-no-header` switch no longer exists** (confirmed absent from the binary's strings table, Chrome 151) — a plain `chrome --headless --print-to-pdf` bakes in the browser's own default header/footer (page title top, URL + page number bottom), regardless of that flag. **Whatever server-side renderer `BB26091208` ends up using must drive the DevTools Protocol's `Page.printToPDF` command directly** (via `puppeteer-core` or an equivalent CDP client, not a bare CLI invocation) and pass `displayHeaderFooter: false` explicitly — confirmed working this session via a raw CDP call (Node's native `WebSocket`/`fetch`, no package install needed, as a proof of concept; a real build would use `puppeteer-core` for a maintained abstraction over the same protocol). | ⬜ Queued — feeds `BB26091204` (Creator Studio wizard + live preview) and `BB26091208` (`weekly-status-brief` archetype); no code ported yet | **Scoped 13 Sep 2026.** Not a design question — the pattern above is concrete and verified, not a decision Sconl needs to make. Filed as PLAN rather than directly amending `build.md` because it's new information those two already-queued rows need folded in before they're picked up, not a row of its own work; whoever picks up `BB26091204`/`BB26091208` should read this row's fix pattern before writing the renderer, then this row can close. | -->
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
| PP26090901 (new, narrowed 9 Sep 2026 — Supabase/Upstash/R2 all removed, self-hosted instead) | **Pre-Cycle 0's external-accounts portion — 5 signups, all need Sconl's own hands (identity/payment details a session doesn't have)** | **Create, in this order (each unblocks specific later work):** (1) **Resend** — ✅ account created (GitHub login), ✅ **`mail.acexoft.com` verified instead of `mail.qspacepress.com`** (per Sconl, 9 Sep 2026 — generalizes the domain across every acexoft-brand service rather than waiting on `PP26090902`'s purchase; free tier's 3-domain limit leaves room to add `mail.qspacepress.com` later for production branding). All 4 DNS records (DKIM, SPF CNAME, SPF TXT, DMARC) added in Cloudflare's `acexoft.com` zone, submitted for verification — status "Pending," Resend's own propagation check can take a few hours, nothing further to do, resolves on its own. (2) **Sentry** — error tracking (blocks Week 4's wiring step, low urgency but a 2-minute signup). (3) **BetterStack** — uptime monitor (same, low urgency, quick signup). (4) **M-Pesa Daraja** — sandbox credentials (likely needs a Safaricom developer account + business registration details; blocks Cycle 3 monetization — worth starting early, historically the slowest approval of the 5). (5) **Stripe** — test mode account (blocks Cycle 3's international-fallback path). | 🟡 In progress — (1) done pending DNS propagation; (2)-(3) next | **Scoped 9 Sep 2026, PLAN session, split out of `BP26090802`'s Week 1 checklist, narrowed twice same day** — first once OCI self-hosting replaced Supabase/Upstash, then again when Cloudflare R2 was replaced by self-hosted MinIO (`BB26090903`/`BB26090904`): R2 required attaching a Cloudflare billing subscription (PayPal on file) before any use, even free-tier — Sconl's call to avoid that before a real user exists, same self-hosting logic already applied to Postgres/Redis/compute. A build session can hand-carry credentials INTO env config once they exist, but cannot create the accounts themselves. **(1)-(3) are the low-friction batch** — no business/bank paperwork, quick signups, done in one sitting; **(4)-(5) deferred** to closer to Cycle 3 (M-Pesa/Stripe both need real business details, not worth front-loading before Cycle 1 even ships). Credentials go into Bitwarden — **recommend a new, dedicated `qspace-press` Secrets Manager project** rather than folding into `isconl`'s existing one, since this is a genuinely separate venture with its own credential surface; not yet confirmed with Sconl, flag before the first secret is actually written. Pulled by build sessions the same key-only-safe way the rest of the fleet does, never pasted into a session directly. -->

