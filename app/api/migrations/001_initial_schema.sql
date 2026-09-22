-- Core entities: users, publications, posts, subscribers.
-- Ported verbatim from the canon doc's Data Model section
-- (work/_arc/qspace-press/canon-canvas/20260325_..._v1_0_0.md).

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- users: creator accounts
CREATE TABLE users (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email       VARCHAR(320) UNIQUE NOT NULL,
  password_hash VARCHAR(60),       -- NULL if OAuth-only
  google_id   VARCHAR(100) UNIQUE, -- NULL if email-only
  created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- publications: one per creator (MVP), many post-launch
CREATE TABLE publications (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id         UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  handle          VARCHAR(30) UNIQUE NOT NULL,  -- URL slug
  display_name    VARCHAR(200) NOT NULL,
  tagline         VARCHAR(300),
  about_html      TEXT,                          -- Rich text about section
  avatar_url      VARCHAR(500),
  cover_image_url VARCHAR(500),
  category        VARCHAR(50),
  custom_domain   VARCHAR(255) UNIQUE,
  custom_domain_verified BOOLEAN DEFAULT FALSE,
  twitter_handle  VARCHAR(50),
  linkedin_url    VARCHAR(300),
  timezone        VARCHAR(50) DEFAULT 'Africa/Nairobi',
  is_monetized    BOOLEAN DEFAULT FALSE,
  monthly_price_kes INT,             -- NULL if not monetized
  monthly_price_usd INT,             -- NULL if not monetized
  stripe_account_id VARCHAR(100),    -- Stripe Connect
  mpesa_paybill   VARCHAR(20),       -- M-Pesa Paybill number
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- posts: content pieces
CREATE TABLE posts (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  publication_id  UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
  title           VARCHAR(200) NOT NULL,
  slug            VARCHAR(250) NOT NULL,
  content_html    TEXT NOT NULL,
  excerpt         VARCHAR(500),
  cover_image_url VARCHAR(500),
  seo_title       VARCHAR(200),
  seo_description VARCHAR(300),
  is_paid_only    BOOLEAN DEFAULT FALSE,
  send_newsletter BOOLEAN DEFAULT TRUE,
  status          VARCHAR(20) NOT NULL DEFAULT 'draft', -- draft | published | scheduled
  published_at    TIMESTAMPTZ,
  scheduled_for   TIMESTAMPTZ,
  newsletter_sent BOOLEAN DEFAULT FALSE,
  newsletter_sent_at TIMESTAMPTZ,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE(publication_id, slug)
);

-- subscribers: readers subscribed to publications
CREATE TABLE subscribers (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  publication_id  UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
  email           VARCHAR(320) NOT NULL,
  name            VARCHAR(200),
  tier            VARCHAR(20) NOT NULL DEFAULT 'free', -- free | paid | comped | unsubscribed
  source          VARCHAR(50),  -- web | import | api | comped
  confirmed       BOOLEAN DEFAULT FALSE,
  confirm_token   VARCHAR(64),   -- UUID token; NULL after confirmation
  subscribed_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  confirmed_at    TIMESTAMPTZ,
  unsubscribed_at TIMESTAMPTZ,
  UNIQUE(publication_id, email)
);

-- Critical for post page rendering
CREATE INDEX idx_posts_publication_slug ON posts(publication_id, slug);
CREATE INDEX idx_posts_publication_published ON posts(publication_id, published_at DESC) WHERE status = 'published';

-- Critical for subscriber queries
CREATE INDEX idx_subscribers_publication_tier ON subscribers(publication_id, tier);
CREATE INDEX idx_subscribers_email ON subscribers(email);
