# Parked — source material for Canvas

**Not integrating into `q-space/press`.** Per Sconl's call (session of 21-22
Sep 2026, `LP26091902`), this branch's work is moving to XKeel's `canvas`
product rather than merging here — the generation engine is extracting out
of Press (see canon D-018, `work/_arc/press/canon-canvas/`).

**Destination row:** `BC26091916` (weekly brief + AI assist).

Do not delete this branch. The Rust in it is real, working-toward-compiling
code — it is moving house, not being discarded. A Canvas build session
picking up `BC26091916` should pull from here rather than rewrite from
scratch.

**Known collision, recorded so it isn't rediscovered the hard way:** this
branch's `app/api/src/main.rs` and `generate/mod.rs`/`generate/handlers.rs`
conflict with `bb26091502-canvas` and `wip/BB26091205-thin-client-api` —
all three register routes in the same place (checked: this branch carries
no colliding `004_*.sql` of its own, unlike canvas/accent-colour/
delivery-metering).
