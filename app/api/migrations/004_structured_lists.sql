-- Structured List Documents v1 (BP26091902). A new document kind alongside
-- Canvas, not a Canvas extension -- see the design note in
-- src/lists/mod.rs for why a separate table beats a field bolted onto
-- Canvas's opaque content_html blob.
--
-- version on each item is the optimistic-lock counter: a PATCH must send
-- the version it read, the update only applies if it still matches, and
-- the counter is bumped on every accepted write. See src/lists/service.rs
-- for the compare-and-bump logic itself.

CREATE TABLE structured_lists (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title       VARCHAR(300) NOT NULL,
  owner_id    UUID NOT NULL REFERENCES users(id),
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE structured_list_items (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  list_id     UUID NOT NULL REFERENCES structured_lists(id) ON DELETE CASCADE,
  position    INTEGER NOT NULL,
  text        TEXT NOT NULL,
  bucket      VARCHAR(50),
  status      VARCHAR(50) NOT NULL DEFAULT 'open',
  owner_name  VARCHAR(200),
  note        TEXT,
  version     INTEGER NOT NULL DEFAULT 1,
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_structured_list_items_list_id ON structured_list_items(list_id);

-- Share-link capability: extends whatever table already backs Canvas's
-- read-only share links (bb26091502-canvas, unmerged as of this writing)
-- with a can_edit flag scoped to the link/token, never the document --
-- see src/lists/mod.rs section 3. Declared here as its own table rather
-- than assumed against a schema that hasn't landed on main yet; fold into
-- the real share-link table once that branch merges.
CREATE TABLE structured_list_share_links (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  list_id     UUID NOT NULL REFERENCES structured_lists(id) ON DELETE CASCADE,
  token       VARCHAR(64) UNIQUE NOT NULL,
  can_edit    BOOLEAN NOT NULL DEFAULT FALSE,
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
