# QSpace Press

A monorepo, three apps:

- **`app/web`** — the public product (Next.js 14 App Router). SEO-indexed publications, a Tiptap-based post editor, newsletter delivery, subscriptions/payments. This is what launches first.
- **`app/api`** — the backend (Rust, Axum). Auth, publications, posts, audience, payments, email, distribution, analytics, storage.
- **`app/mobile`** — a Flutter companion app. Explicitly **post-launch / inactive** per the canon doc's MVP exclusion list — scaffolded and preserved, not under active development. Do not start building against it early.

See `work/_arc/qspace-press/canon-canvas/` for the full technical and product canon (architecture, data model, deployment plan) — this README is orientation, not the spec.

## Local dev

`docker compose up` boots the whole stack (`app/web`, `app/api`, Postgres, Redis) on dedicated ports — see `docker-compose.yml` and `.env.example`. Chosen deliberately over the canon doc's original managed-Supabase/managed-Upstash assumption, so local dev never depends on external accounts.

## Status

Pre-Cycle 0 — most of `app/api` is scaffolding with no served interface yet
(`payments`, `posts`, etc. wait on `db.rs` connecting Postgres). One real
exception: `generate::` (the document-generation engine — 9 archetypes, 4
renderers, ported from iSconl `scope`'s JS engine per `BB26091203`) is
substantial and is served today at `POST /api/generate` /
`GET /api/generate/archetypes` (`BP26091906`) — it needs no database, so it
didn't have to wait for the rest of the stack. Corrected 19 Sep 2026: this
line previously said "no product code shipped yet," which was stale and
misled in the opposite direction from iSconl canon §5 (which already
assumed these capabilities had moved here).
