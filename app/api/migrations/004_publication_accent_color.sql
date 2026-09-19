-- BB26091501: per-brand default accent for generated documents (Creator
-- Studio archetypes, e.g. weekly-status-brief). Reuses the existing
-- `publications` row a document's target already resolves to -- no
-- parallel "brand settings" table. NULL means "no brand default set";
-- the generation engine falls back to its own house default
-- (`generate::style::DEFAULT_ACCENT`) when this is NULL, same as when a
-- document itself carries no per-document override.

ALTER TABLE publications
  ADD COLUMN accent_color VARCHAR(6); -- bare hex, e.g. '2F5496' -- no leading '#', same convention as generate::style::STYLE's colors

ALTER TABLE publications
  ADD CONSTRAINT publications_accent_color_is_hex
  CHECK (accent_color IS NULL OR accent_color ~ '^[0-9a-fA-F]{6}$');
