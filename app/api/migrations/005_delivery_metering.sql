-- Delivery metering (BB26091206). Meter newsletter delivery on subscribers
-- delivered to, not posts written -- canon: work/_arc/press/canon-canvas/
-- 20260915_project_development_canonical_qspace_press_v2_1_0.md, "Pricing
-- Strategy" + Decision Log D-008.
--
-- Delivery tier (this file) and revenue share (Feature 2 / payments take
-- rate, not built yet -- see BB26091206's own findings) are independent
-- billing dimensions. This migration only ever touches delivery. Nobody
-- pays twice for the same thing.

-- publications.delivery_plan: which delivery allowance a publication is
-- on. Defaults to 'free' (1,000 subscribers, see tiers.rs) for every
-- existing and new publication until a creator/operator changes it.
ALTER TABLE publications
  ADD COLUMN delivery_plan VARCHAR(20) NOT NULL DEFAULT 'free'
    CHECK (delivery_plan IN ('free', 'growth', 'scale', 'institutional')),
  ADD COLUMN delivery_plan_updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- institutional_allowance_override: Institutional pricing is "Negotiated"
-- per the canon pricing table -- unlike the other three tiers it has no
-- fixed subscriber cap, so it needs a per-publication number instead of a
-- constant. NULL and unused for every other plan.
ALTER TABLE publications
  ADD COLUMN institutional_allowance_override INT;

-- newsletter_send_jobs: one row per post's send job. This is the "email
-- queue" tier enforcement sits in front of -- a post is never sent by
-- directly fanning out to email_deliveries; it is always enqueued here
-- first, where the billable subscriber count and plan are evaluated and
-- recorded at send time.
--
-- held_over_limit is the honest-degradation path: the row still exists
-- (nothing is silently dropped), the send simply does not go out until
-- try_release_held_job() (worker.rs) confirms the publication is back
-- within its allowance.
CREATE TABLE newsletter_send_jobs (
  id                         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  post_id                    UUID NOT NULL UNIQUE REFERENCES posts(id) ON DELETE CASCADE,
  publication_id             UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
  status                     VARCHAR(20) NOT NULL DEFAULT 'pending',
    CHECK (status IN ('pending', 'held_over_limit', 'sending', 'completed', 'failed')),
  billable_subscriber_count  INT NOT NULL,  -- confirmed, non-unsubscribed count at enqueue time
  delivery_plan              VARCHAR(20) NOT NULL,  -- plan snapshot at enqueue time
  delivery_allowance         INT NOT NULL,  -- numeric allowance snapshot at enqueue time
  held_at                    TIMESTAMPTZ,
  held_reason                TEXT,
  creator_notified_at        TIMESTAMPTZ,
  enqueued_at                TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  started_at                 TIMESTAMPTZ,
  completed_at               TIMESTAMPTZ,
  created_at                 TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at                 TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Critical for the send worker's queue scan and for the held-job release sweep
CREATE INDEX idx_newsletter_send_jobs_status ON newsletter_send_jobs(status, enqueued_at);
CREATE INDEX idx_newsletter_send_jobs_publication ON newsletter_send_jobs(publication_id, enqueued_at DESC);
