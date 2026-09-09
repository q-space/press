-- Email delivery tracking and post-level analytics. Ported verbatim from
-- the canon doc's Data Model section. Depends on 001/002 (posts, subscribers,
-- publications).

-- email_deliveries: newsletter send tracking per post per subscriber
CREATE TABLE email_deliveries (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  post_id         UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  subscriber_id   UUID NOT NULL REFERENCES subscribers(id) ON DELETE CASCADE,
  resend_message_id VARCHAR(100),
  status          VARCHAR(20) NOT NULL DEFAULT 'queued', -- queued | sent | delivered | opened | bounced | failed
  sent_at         TIMESTAMPTZ,
  opened_at       TIMESTAMPTZ,
  bounced_at      TIMESTAMPTZ,
  bounce_type     VARCHAR(20),  -- hard | soft
  UNIQUE(post_id, subscriber_id)
);

-- analytics_events: post-level engagement tracking
CREATE TABLE analytics_events (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  post_id         UUID REFERENCES posts(id) ON DELETE CASCADE,
  publication_id  UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
  event_type      VARCHAR(50) NOT NULL, -- post_view | email_open | email_click | subscribe | unsubscribe | paid_subscribe | whatsapp_share
  session_id      VARCHAR(64),   -- anonymous session for dedup
  source          VARCHAR(50),   -- organic | whatsapp | twitter | email | direct
  country_code    CHAR(2),
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
-- Note: No PII stored in analytics_events. subscriber_id intentionally excluded.

-- Critical for newsletter queue
CREATE INDEX idx_email_deliveries_post_status ON email_deliveries(post_id, status);

-- Critical for analytics aggregation
CREATE INDEX idx_analytics_publication_type_date ON analytics_events(publication_id, event_type, created_at DESC);
CREATE INDEX idx_analytics_post ON analytics_events(post_id, created_at DESC);
