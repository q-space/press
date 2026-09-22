# Parked — source material for Canvas

**Not integrating into `q-space/press`, even though it merges cleanly into
`dev` today.** Per Sconl's call (session of 21-22 Sep 2026, `LP26091902`),
this branch's work is moving to XKeel's `canvas` product rather than merging
here — the generation engine is extracting out of Press (see canon D-018,
`work/_arc/press/canon-canvas/`). Merging cleanly is not the same question as
where the work belongs once Canvas exists as a separate product.

**Destination row:** `BC26091905` (theme model).

Do not delete this branch. The Rust in it is real, working-toward-compiling
code — it is moving house, not being discarded. A Canvas build session
picking up `BC26091905` should pull from here rather than rewrite from
scratch.

**Known collision, recorded so it isn't rediscovered the hard way:** its
`004_publication_accent_color.sql` migration collides on number with three
other branches' own `004_*` files (`bb26091502-canvas`'s `004_canvas.sql`
and, before this pass, `bb26091206-delivery-metering`'s — now renumbered to
`005_delivery_metering.sql`, see that branch). Only `004_structured_lists.sql`,
already on `dev`, is real. Expect to renumber this one too when it actually
gets pulled into Canvas.
