-- canvases: content-type-agnostic, live-editable HTML artifacts (BB26091502).
--
-- Deliberately generic: a Canvas is a title and rendered HTML, nothing
-- else. It must never gain a field that only makes sense for one caller's
-- content (a "course_id", a "module_number", anything iSpark-shaped) --
-- see BB26091502's row note for why: iSpark stays its own product per
-- Sconl's 15 Sep ruling, and a course-specific column here would quietly
-- re-implement the fold he ruled out. Any product (the archetype wizard
-- today, iSpark's own rendering pipeline later) publishes through this
-- the same way: hand it a title and rendered HTML.
CREATE TABLE canvases (
  id            UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  title         VARCHAR(200) NOT NULL,
  content_html  TEXT NOT NULL,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
