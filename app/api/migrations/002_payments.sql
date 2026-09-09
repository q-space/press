-- Subscriptions and payments. Ported verbatim from the canon doc's Data
-- Model section. Depends on 001_initial_schema.sql (subscribers, publications).

-- subscriptions: paid subscription records
CREATE TABLE subscriptions (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  subscriber_id   UUID NOT NULL REFERENCES subscribers(id) ON DELETE CASCADE,
  publication_id  UUID NOT NULL REFERENCES publications(id) ON DELETE CASCADE,
  payment_method  VARCHAR(20) NOT NULL, -- mpesa | stripe
  status          VARCHAR(20) NOT NULL DEFAULT 'active', -- active | cancelled | expired | past_due
  price_kes       INT,           -- if mpesa
  price_usd       INT,           -- if stripe (cents)
  stripe_sub_id   VARCHAR(100),  -- Stripe subscription ID
  current_period_start TIMESTAMPTZ NOT NULL,
  current_period_end   TIMESTAMPTZ NOT NULL,
  cancelled_at    TIMESTAMPTZ,
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- payments: individual payment transactions
CREATE TABLE payments (
  id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  subscription_id UUID REFERENCES subscriptions(id),
  subscriber_id   UUID NOT NULL REFERENCES subscribers(id),
  publication_id  UUID NOT NULL REFERENCES publications(id),
  payment_method  VARCHAR(20) NOT NULL, -- mpesa | stripe
  amount_kes      INT,
  amount_usd      INT,          -- cents
  platform_fee_kes INT,         -- 10%
  net_to_creator_kes INT,       -- 90%
  status          VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending | confirmed | failed | refunded
  mpesa_checkout_request_id VARCHAR(100),
  mpesa_receipt_number      VARCHAR(50),
  stripe_payment_intent_id  VARCHAR(100),
  error_message   TEXT,         -- failure reason if status = failed
  created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  confirmed_at    TIMESTAMPTZ
);

-- Critical for payment reconciliation
CREATE INDEX idx_payments_mpesa_checkout ON payments(mpesa_checkout_request_id);
CREATE INDEX idx_payments_publication_status ON payments(publication_id, status, created_at DESC);
