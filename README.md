# QSpace Press

A monorepo, three apps:

- **`app/web`** — the public product (Next.js 14 App Router). SEO-indexed publications, a Tiptap-based post editor, newsletter delivery, subscriptions/payments. This is what launches first.
- **`app/api`** — the backend (Rust, Axum). Auth, publications, posts, audience, payments, email, distribution, analytics, storage.
- **`app/mobile`** — a Flutter companion app. Explicitly **post-launch / inactive** per the canon doc's MVP exclusion list — scaffolded and preserved, not under active development. Do not start building against it early.

See `work/_arc/qspace-press/canon-canvas/` for the full technical and product canon (architecture, data model, deployment plan) — this README is orientation, not the spec.

## Local dev

`docker compose up` boots the whole stack (`app/web`, `app/api`, Postgres, Redis) on dedicated ports — see `docker-compose.yml` and `.env.example`. Chosen deliberately over the canon doc's original managed-Supabase/managed-Upstash assumption, so local dev never depends on external accounts.

## Status

Pre-Cycle 0 — repo scaffolding in progress, no product code shipped yet.
