# Backlog — legend, category scheme, and standing rules

<!-- LIVE DOCUMENT — read this first, it explains how every other file in
     backlog/ works. Generic template, adapted from isconl's backlog/
     system (`_kit/work/dev/iSconl/_next/backlog/STANDING-RULES.md`) on
     18 Aug 2026 — isconl-specific hard rules (Jira, Viva doc naming, WORK
     task mirroring) stripped out; the category scheme and mechanics below
     are the shared core, meant to be reused as-is across every project. -->

## Legend (status symbols, used in every category file)

| Symbol | Meaning |
|--------|---------|
| 🔴 | Blocked — cannot proceed without external action (including a Sconl-only decision) |
| 🟡 | In progress |
| 🟢 | Done this session |
| ⬜ | Queued |
| 🔵 | Suggested / not yet committed to |
| 🧊 | Shelved — paused, not abandoned |

## Category scheme

**Five files, mapped to model tier** (consolidated from eight 15 Sep
2026, per Sconl: "let us provision for 'subsets' in the list... so that
we have only five main sessions"). Every task still carries exactly one
letter — each one is an action verb, naming what the *next* session on
that row should DO, not a noun describing its state. A task can be
re-lettered as it matures (a **PLAN** row that gets a design pass
becomes a **BUILD** row; a **BUILD** row that turns out to need more
thought drops back to **PLAN**) — move the row to the matching file
when that happens, don't just relabel it in place.

| Letter | Verb | File | Meaning |
|--------|------|------|---------|
| **P** | **PLAN** | `plan.md` | Real and wanted but needs a design/scoping pass first — **including anything blocked purely on the owner's input/approval**, flagged 🔴. There is no separate "decide" file — a blocked decision is just a PLAN row waiting on an answer. **This is what Plan sessions work from and add to.** |
| **B** | **BUILD** | `build.md` | Scoped and ready to build now — a builder session can pick this up with no further design work. Code/feature builds, and (as the **F — Fix** subset, see "Subset structure" below) bug fixes and found gaps — both are code, generation and repair alike, run at the same model tier. |
| **W** | **WORK** | `work.md` | Scoped and ready to execute now, same bar as BUILD, but NOT a code/feature build — writing docs, content, non-code deliverables. |
| **L** | **LIGHT** | `light.md` | Low-risk, mechanical work safe for a lighter model to execute unattended: **R — Refine** (non-critical UI/UX polish, nothing broken, just not as good as it could be) and **O — Organize** (folder/file moves, repo restructuring, renames, refactors — not a bug, not new capability, this is about where things live) as subsets, see "Subset structure" below. |
| **C** | **CONTENT** | `content.md` | Optional — only for projects doing course/track content development. Omit this file entirely for projects that don't need it. |

**`hands.md` (letter `H`) stays OUTSIDE this five-file count** — it's a
hand-tended registry (credential rotation plus anything else only the
owner's own hands can resolve), not a model-executable backlog, so it
needs no migration and doesn't count against "five lists." See "Hands —
only the owner can resolve this" below.

### Subset structure

`build.md` and `light.md` each hold two former standalone categories.
**The file merges; the letter does not** — an `F`-lettered Fix row
stays `F` inside `build.md`, an `R`/`O`-lettered row stays `R`/`O`
inside `light.md`. IDs are never re-minted for this reason alone (only
the PLAN-handoff re-mint rule below still applies), so every existing
cross-reference keeps resolving straight through the merge.

**Express a subset with a `## Subset: <Name> (<Letter>)` heading**,
grouping that subset's rows together under it rather than interleaving
them with the parent category's own rows — e.g. `build.md` reads
top-level BUILD (`B`) rows first, then a `## Subset: Fix (F)` heading
with its own rows beneath. This keeps two things true at once: a
session can still work "just the fix rows" without a filter, and each
subset's row count stays visible on its own line in the queue inventory
matrix (`CLAUDE.md` §19), not folded invisibly into the parent's count.

**When performing a category merge (or any file consolidation): never
blindly overwrite a file that might already have content — always diff
first, then append or merge, never replace wholesale** (learned 15 Sep
2026, `BO26091401`'s own migration: a session merging `fix.md` into
`build.md` across ~20 projects assumed every project's `build.md` was
the empty scaffold and overwrote two of them wholesale with a fresh
template — one held a live, unfinished row that would have been
destroyed outright, caught only because the drive-wide row count moved
and didn't reconcile. **A row-count check alone is not sufficient
protection, and the case that slips through it is the more dangerous
one:** a second project's `build.md` had zero live rows but carried
real provenance comments (where its rows came from, when they were
shelved) — a count-based check passes cleanly on zero rows before and
after, while the comments are destroyed anyway, because a count cannot
see content it doesn't count. The only reliable practice is to read (or
diff) the file's actual current content before writing to it, every
time, even when — especially when — you expect it to be empty.

### Task IDs

Every row gets an intuitive, permanent ID:

```
B<letter><YYMMDD><NN>
```

- `B` — fixed, marks it as a backlog ID.
- `<letter>` — the one-letter category the row lived in **when the ID was
  assigned** (`P`/`B`/`W`/`L`/`C`, or a subset letter — `F` inside
  `build.md`, `R`/`O` inside `light.md` — per "Subset structure" above).
- `<YYMMDD>` — the date the ID was assigned.
- `<NN>` — 2-digit sequence within that letter+date, starting at `01`.

**IDs get re-minted on PLAN handoff.** When a PLAN session finishes scoping
a `plan.md` row and hands it off to its destination file, the ID's letter
is rewritten to match that destination — a `BP...` row moving to
`build.md` becomes a `BB...` row (next free sequence number for that
date), one moving to `work.md` becomes `BW...`, and so on. When an ID is
rewritten, update every cross-reference to the old ID across all backlog
files (grep for it) in the same commit.

`done.md` holds recently shipped work, kept for history/citation (commit
SHAs, what a fix actually was). Nothing gets deleted from a category file
when it's finished — it moves to `done.md` instead.

### Title column (standing convention, added 18 Aug 2026)

Every row, in every category file (including `done.md`/`done-archive/`),
carries a dedicated **Title** column immediately after the ID column —
`| # | Title | Task | Notes |` for `plan.md` (no `Status` column there),
`| # | Title | Task | Status | Notes |` for `build.md`/`work.md`/
`light.md`/`content.md` (subset sections included). The title is **5-7 words, bold, a succinct
standalone summary** of the row — Sconl's own words: "a clear succinct
preview of each item." It is NOT the first sentence of the Task cell
truncated; it's a distinct, deliberately short label a reader can scan a
whole file by, without reading the Task cell at all. Assign the title the
moment a row is written (new inflow, PLAN handoff, or a fresh BUILD/WORK/
FIX/REFINE/ORGANIZE find) — never leave a row without one "for later."

### Done-list retention

`done.md` must stay short enough to skim in one sitting. Permanent history
lives in `done-archive/YYYY-MM.md` files instead.

- **Trigger:** whenever `done.md` holds more than ~3 dated session-blocks,
  or reading top-to-bottom stops being a quick skim (rough guide: past
  ~150 lines) — move the oldest dated block(s) out.
- **How:** append the oldest block(s) to `done-archive/<year>-<month>.md`
  (create it if this is the first archived block for that month), then
  replace that block in `done.md` with a one-line pointer.
- **Never archive out of `plan.md`/`build.md`/`work.md`/`light.md`/
  `content.md`** (subset sections included) — those stay lean because
  finished rows move OUT to `done.md`, not because old unfinished rows
  get hidden away.

## What a Plan session does (never executes)

A PLAN session's only output is an updated backlog — it never writes
application code, never edits project files outside `_next/backlog/`,
and never edits `_arc/` canon docs directly (it can flag that canon needs
updating once real work lands, but the edit itself happens in whichever
session does that work). If a PLAN session catches itself about to
implement something, that's the signal to stop and write a row instead.

**`plan.md` is a routing hub, not a holding pen.** Every other session —
BUILD, FIX, REFINE, ORGANIZE, Chat — that hits something real but unscoped
mid-session routes it INTO `plan.md` as new inflow. PLAN's job is the
reverse direction: take what's sitting in `plan.md` and work it into a
fully scoped row, then write it OUT to whichever file now fits.

**Zero-ambiguity handoff bar.** A row only leaves `plan.md` once it is
concrete enough that the session picking it up needs to make no further
judgment calls — exact scope, exact files/paths/APIs touched, exact
acceptance criteria. If executing the row would still require someone to
decide something PLAN could have decided (or asked about) instead, the
row isn't done — keep scoping. "Been discussed" isn't the bar; "a builder
could pick this up cold" is.

**Pre-think every decision before it's asked, don't wait to trip over
it.** Before a row is written as scoped, PLAN's job is to actively
enumerate every decision point the row is hiding — every "well, it
depends on X." Each one gets either resolved from what's already known or
discoverable, or surfaced to Sconl as an explicit, elaborated question
(same depth as the DECODE pipeline's "present decisions elaborately"
rule below) — before the row is marked ready. A row should never reach
BUILD/WORK/FIX/ORGANIZE carrying a hidden unresolved decision that only
surfaces mid-execution; that decision belonged in PLAN, found and asked
up front, not discovered by whoever executes.

**Decisions live inside `plan.md`, flagged 🔴.** Any session that hits
something blocked purely on Sconl's call adds (or flags) a `plan.md` row
🔴 and moves straight on — never stall a session waiting for an answer
that isn't coming this session. Once Sconl answers, the answer is written
back into that same row, the 🔴 flag clears, and the row either resolves
to `done.md` or continues its normal outflow. There is no separate
decision file — a blocked decision is just a PLAN row waiting on an
answer.

**Correct routing, every time.** PLAN letters a row honestly against the
category table — BUILD (code/feature), WORK (non-code deliverable), FIX
(something broken), REFINE (polish), ORGANIZE (moves/restructuring) — and
catches anything that doesn't belong in a backlog row at all (pure
conversation, a one-off answered question). A misrouted row is a PLAN
defect, fixed the moment it's noticed — moved to the file it actually
belongs in, not left "close enough."

**How to apply:** when a session opens as Plan, pull the current
`plan.md` as the starting queue — don't make Sconl restate it. Walk items
one at a time: discuss and pre-think until every decision point is either
resolved or asked, then immediately write the row to its right
destination file (don't batch every item to the end).

## "Execute the X backlog" — autonomous, ordered, top-to-bottom

When Sconl says "execute the build backlog," "run through work.md," "execute
the fix backlog," or equivalent for any category, that means: open the
named category file and work its rows **top to bottom, autonomously, one
at a time, without stopping to ask a question per row.** This is a
standing command shape, not something to re-clarify each time it's said.

**This is only possible because of the zero-ambiguity handoff bar
above — PLAN already resolved every decision before a row got here.** If
an executing session hits a row it can't act on without asking Sconl
something, that is a PLAN defect discovered late, not a normal part of
execution:
- Stop on that row, not the whole run — log what's actually blocking it
  as a fresh 🔴-flagged `plan.md` row (with a real ID) so PLAN catches
  and fixes the scoping gap next time, and move on to the next row in
  the file.
- Never silently skip a row and call the run "done" — either it executed
  clean, or it's now flagged in `plan.md` with a note on this row saying
  why it bounced back.

**Row order in every category file IS execution order, not just a
priority hint.** The top row is the next thing an autonomous run does;
the bottom row is last. This means:
- **PLAN orders on handoff, not just letters and scopes.** When a row
  moves from `plan.md` into `build.md`/`work.md`/`light.md`/`content.md`
  (or a subset section within `build.md`/`light.md`), it goes in at the position that reflects real
  execution order against what's already there — dependencies first
  (a row nothing else needs waits behind a row it depends on), then
  urgency × impact ÷ complexity as the tiebreak. Dropping a new row at
  the bottom "to sort later" is not acceptable — order it correctly at
  handoff time.
- **Reorder existing rows whenever priority genuinely changes** — this
  was already a standing rule (see "Standing rules" below); this section
  makes explicit *why* it matters now: a stale order isn't just
  cosmetically wrong, it's the literal execution sequence a bare
  "execute the X backlog" command will follow.
- **A row that's blocked on something external (🔴) still holds its
  correct position** — an autonomous run reaching a blocked row skips it
  (log why, per above) and continues to the next row, rather than the
  blocked row needing to be manually sorted to the bottom first.

## The DECODE pipeline — decisions never block a session

The standing path a Sconl-only call takes from "found" to "resolved and
written back":

1. **Never stall a session on a pending decision.** If BUILD, FIX,
   REFINE, ORGANIZE, PLAN, or Chat hits something genuinely blocked
   purely on Sconl's call, add (or flag) a `plan.md` row 🔴 (with a real
   `BP<YYMMDD><NN>` ID) and move straight on to the next item in that
   session's own queue.
2. **Present decisions elaborately, not tersely.** When a 🔴 `plan.md`
   row actually goes in front of Sconl for an answer, the framing must
   carry enough on its own that he doesn't have to reconstruct context:
   what's already built/verified vs. what's gated on this call, every
   real option on the table (not just yes/no), and the concrete tradeoff
   of each. A one-line question is a sign the row wasn't scoped enough —
   that's PLAN's job to fix before it reaches Sconl.
3. **IDs are minted once and never re-minted**, except on the PLAN
   outflow handoff rule above.

## Shelving — pausing without losing

`shelved.md` holds work that's real, still wanted, and deliberately paused
— not done (nothing here shipped), not abandoned (nothing here is
deleted), and not a PLAN row (it isn't blocked on a decision or scoping
gap, it's blocked on priority). Use it when a whole row, a whole project,
or a whole category of a project's work is being set aside on purpose so
it stops showing up in normal browsing and "execute the X backlog" runs,
while staying fully recoverable.

**To shelve a row:** cut it from its active file (`plan.md`/`build.md`/
`work.md`/`light.md`/`content.md`, subset sections included) and paste it verbatim under
that file's `### From <file>.md` heading in `shelved.md` (create the
heading if this is the first row shelved from that file). Prepend a
one-line preface above the row: `**Shelved <date>:** <reason>.` Do **not**
re-mint the ID — shelving is a pause, not a promotion/handoff, so the ID
keeps its original letter and date.

**To shelve an entire project:** move every live row out of every active
category file into that project's `shelved.md`, grouped by originating
file, with one reason/date note at the top of `shelved.md` covering the
whole project (no need to repeat the same reason on every row). Leave
each emptied category file's header and table row intact — an empty
table, not a deleted file.

**To unshelve:** move the row (or every row, for a whole-project
unshelve) back to its originating file, dropped in at the position that
reflects *current* priority against what's already there (per the
row-order-is-execution-order rule below) — not necessarily where it sat
before. Remove the shelving preface line once restored.

**`shelved.md` is excluded from "execute the X backlog."** An autonomous
run works `build.md`/`work.md`/etc top to bottom as normal; `shelved.md`
is never one of those files, since a shelved row is out of scope by
definition until someone unshelves it.

**The drive-wide rollup (`_kit/backlog/shelved.md`) gives a single place
to see everything currently paused across every project** — same
generated-rollup mechanics as every other category file (see
`_kit/backlog/STANDING-RULES.md`), never hand-edited.

## What a Light session does (Refine + Organize subsets)

Works `light.md` — both its subsets, top-level rows first, then each
`## Subset: <Name> (<Letter>)` section in turn, same scoped/
ready-to-execute bar as BUILD/WORK. **Refine (`R`)** is non-critical
UI/UX polish — nothing broken, just not as good as it could be, no
special care rules beyond the standing ones. **Organize (`O`)** is
folder/file moves, repo restructuring, renames, refactors where
behavior already works and this is purely about where things live —
with two extra care rules, because moves are harder to walk back than
most other categories' work:
1. **Copy, verify, then delete** — never move-and-trust. Copy to the new
   location, verify (diff/checksum or a manual spot-check) it's complete
   and correct, and only then remove the original.
2. **A folder that can't yet be safely deleted stays flagged, not
   forced.** If something still reads from the old location or ownership
   is unclear, leave the row open with a note on what's blocking cleanup.
3. **A rename sweep must cover `.gitignore` allow-lists, not just prose
   references** (learned 15 Sep 2026, `BO26091403`'s aquifer-content →
   aquifer rename: every markdown cross-reference was grepped and fixed
   correctly, but `.gitignore`'s own allow-list entries still pointed at
   the old path — so every NEW file created under the renamed directory
   afterward was silently untracked, not even showing as untracked in
   `git status`, until the gap was found by accident days later). A
   grep for the old name across `*.md`/`*.tsv` does not find a
   `.gitignore` hit unless `.gitignore` itself is included in that grep
   — check it explicitly, every time a tracked directory moves.

## Secret checks — key-only, never value-dumping (added 15 Sep 2026)

Full incident history and the complete standing rule live in the relay
root `CLAUDE.md` §17 (drive-wide) — read that before running any
existence check against a secret. Summary, kept here so the rule is
visible from inside this project without leaving it:

**Any check for whether a secret/credential exists must use a command
whose output structurally cannot contain the value** — not a
value-containing command with a redaction step bolted on. This applies
to env vars, Bitwarden/`bws` secrets, token files, shell profiles
(`.bashrc`/`.zshrc`/`.profile`), `.env` files, and SSH/rclone/git
configs — any file that may hold a `KEY=value`/`key: value` line, not
only the obvious "secrets manager" commands.

- Env var presence: `[ -n "$VAR" ] && echo set || echo "not set"` (bash)
  / `if ($env:VAR) { "set" } else { "not set" }` (PowerShell) — never
  `echo $VAR` through anything, redacted or not.
- `bws` secret presence/name: `bws secret list | jq -r '.[].key'` — never
  a bare `bws secret list` or `bws secret get <name>`, both of which
  return the full plaintext value unconditionally.
- Whether a `KEY=` line already exists in a file: `grep -q "KEY" file`
  (exit code only) or `grep -c "KEY" file` (count only) — never bare
  `grep "KEY" file` or `grep -n "KEY" file`, both of which print the
  matched line including its value.
- If a command's output shape is unfamiliar, check what it returns
  (docs, `--help`) before running it against secret data.

If a value leaks into a transcript anyway: stop, tell Sconl immediately
which secret(s) leaked and where, then log it in `hands.md` below,
flagged for rotation — don't quietly move on.

## Hands — only the owner can resolve this (added 6 Sep 2026 as Opsec, merged into Hands 15 Sep 2026)

`hands.md` holds every row that only the owner's own hands can resolve —
not a scoping gap, not a decision waiting on input, but work a session
physically cannot do no matter how well it's scoped: an account only
they control, a machine a session can't reach, an admin panel a session
has no credentials for, a payment only they can make, or (this file's
original scope, before the 15 Sep 2026 merge) touching a live secret —
rotating an API key/token, changing a passphrase, re-authenticating an
OAuth account, revoking and regenerating a credential. **It exists so
every other category file stays fully executable by a session with zero
owner-only steps in the way** — before this split existed, a credential
rotation or any other hands-only blocker had nowhere standing to go and
either sat stranded in a FIX/BUILD file (breaking "execute the backlog
top to bottom, no human needed") or got shuffled into `plan.md` Tier 2
alongside unrelated design decisions, diluting that file's actual
purpose.

**The owner tends this file personally.** A session's job here is to
keep it well-stocked and precise — write the row, verify the steps are
exact and correct — not to execute the action itself, the same boundary
that already applies to entering a password.

**Row format** (`| # | Title | What it needs from you | Exact steps | Status | Notes |`):
- **What it needs from you** — for a credential row, the key/service
  itself and why it's being rotated now; for a general hands row, the
  specific account/machine/admin-access/payment only the owner can
  supply.
- **Exact steps** — real commands verbatim for a terminal operation
  (Bitwarden `bws` key-only lookups, `git`, `openssl rand`, etc.), or a
  numbered click-path for a web-console/browser operation. Precise
  enough the owner can execute top to bottom without reconstructing the
  process themselves. **Never a real secret value in this file** — every
  standing secret-handling rule above (key-only checks, no value-dumping
  commands) applies here too, with extra force, since this file's whole
  purpose is being read and acted on directly.
- `H<DomainLetter><YYMMDD><NN>` ID scheme, same as every other category
  (see "Task IDs" above) — re-mint when a row moves here from another
  category, same as any other cross-file move.

**Standing routing rule, for every category and every session:** any row
that turns out to be something only the owner's own hands can resolve —
an account, a machine, an admin panel, a payment, or a
credential/key/password/token rotation — found while fixing, scoping,
or building anything, moves to `hands.md` instead of sitting in
`build.md`'s Fix subset or `plan.md`. Includes OAuth re-authentication
(a dead refresh token, a revoked consent) even though no "rotation" in
the strict key-rotation sense is happening — the common thread is "only
the owner's own credentials/hands can resolve this," not the exact
mechanism.

## Error capture

Every session, in any category, routes problems it hits to `build.md`'s
Fix subset (`## Subset: Fix (F)`) — not BUILD-session-exclusive, this is
what happens the moment something is found broken, regardless of what
the session opened to do.

- **If it can be fixed within the current session** without derailing its
  actual purpose: fix it inline, log straight to `done.md`.
- **If it can't be fixed now:** add a row to `build.md`'s Fix subset
  describing what broke and how it was found, and continue with the
  session's original purpose.

## Concurrency protocol

- **Before editing:** pull the latest from `origin` so you're not writing
  on stale state.
- **After editing a category file:** commit and push as soon as the edit
  is meaningful — don't batch every category's changes into one
  end-of-session commit.
- **If two sessions edit the same file concurrently:** these are markdown
  tables — a conflict is almost always two different rows near the same
  line. Resolve by keeping both rows, never silently drop one.
- **`index.lock: File exists` means a peer is mid-commit** — wait ~10s
  and retry, never delete the lock file yourself.
- **Commit by pathspec, never a bare `git commit`** (added 15 Sep 2026,
  `CLAUDE.md` §20, from a real incident: a bare `git commit` on a shared
  working tree silently swept another session's staged-but-uncommitted
  `git mv` renames into an unrelated commit — nothing was lost, but the
  history misattributed real work, and the same mechanism would have
  committed a half-finished change just as willingly). `git status`
  before committing helps but races against a peer staging something in
  the gap between your check and your commit — the structural fix is to
  never let the commit reach further than you intend:
  ```
  git commit -m "..." -- <path1> <path2> ...
  ```
  This commits only the given paths regardless of what else sits in the
  shared index. Same principle as the key-only secret checks above —
  prefer the command whose shape makes the bad outcome impossible over
  the one that needs you to remember a precaution.
- **A stray `git checkout`/`reset --hard` reaches every session's
  uncommitted work** on a shared working tree — see "Concurrent
  sessions — worktree per session" immediately below for the structural
  fix.

### Concurrent sessions — worktree per session (added 4 Sep 2026)

**Any session about to do real work on a shared-checkout repo first
creates its own `git worktree` rather than working directly in the
canonical checkout.** The canonical checkout stays on its default branch
and is never `checkout`'d or `reset --hard`'d by an individual working
session.

```
git worktree add ../<repo>-<session-id> <branch>
```

**Why:** a concurrent session running `git checkout` + `git reset --hard`
against another session's shared working tree can silently destroy its
uncommitted work — a worktree per session makes that class of collision
structurally impossible instead of relying on sessions never colliding.
A worktree is disk space only, no new tooling to install.

## Standing rules (carry forward every session)

1. **After completing a task:** move its row from the active category file
   to `done.md` with a commit SHA or file reference.
2. **After receiving a new task:** add it to the right category file
   (letter it honestly — most new asks start as **P**, including anything
   blocked on the owner's decision, flagged 🔴 — not **B**, unless it's
   genuinely a one-sitting build).
3. **Reorder rows within a file** after each session so the top always
   reflects current priority (urgency × impact ÷ complexity).
4. **Every session** must update the relevant backlog file(s) at start and
   end of session, and sync per the concurrency protocol above.
5. **Zero client/tenant/personal PII in code repos.** Personal or
   sensitive data belongs in OneDrive or the private `_kit` repo, never in
   an open-source or otherwise shared code repo.
6. **Route every problem found to `build.md`'s Fix subset**, regardless
   of session category — see "Error capture" above.
7. **Read `work/_arc/<project>/canon-canvas/SESSION-RECORD.md` before
   starting substantive work on this project, and update it in the same
   session as any backlog change** — `CLAUDE.md` §15 (added 15 Sep 2026,
   `OI26091201`). Reading it is the first step of the work, not optional
   context-gathering: it carries what the project currently is, what was
   decided and **why**, and what is worth watching. **The record is
   additive — nothing in it is ever deleted.** A superseded `Current
   State` moves into `Historical States` under its date; the
   `Decisions & Reasoning` log keeps entries even after a decision is
   reversed, and records options **considered and rejected** with the
   reason, so a rejected option does not get re-proposed every few months.
   Past states are context, not clutter.
