// app/api/src/push/store.rs
//
// CHANGELOG:
// - BB26091207: initial in-memory push subscription store.
//! In-memory push subscription store (BB26091207).
//!
//! Same stage the rest of `app/api` is at: `db.rs` isn't wired into
//! `main.rs` yet (Week 2, once `migrations/001_initial_schema.sql` and
//! SeaORM entities land -- see `db.rs`'s own doc comment), so there is no
//! Postgres to persist into today. Rather than leave `/push/subscribe` as
//! an unbuilt stub like `posts::handlers` or block this row on Week 2's
//! DB work, this keeps subscriptions in an in-process `HashMap` behind a
//! `Mutex` -- real enough to exercise the whole subscribe flow end to end
//! (browser -> Next BFF -> here), explicitly NOT durable across restarts.
//!
//! Follow-up once `db.rs` is wired: replace this with a
//! `push_subscriptions` table (endpoint unique, handle, p256dh, auth,
//! subscribed_at) and swap this module's two methods for SeaORM calls --
//! the `PushSubscriptionRecord` shape below is deliberately already
//! table-row-shaped so that swap doesn't touch `handlers.rs` at all.

use std::collections::HashMap;
use std::sync::Mutex;

// NOTE: deliberately no `Serialize` derive -- `chrono`'s dependency here
// (see Cargo.toml) is `default-features = false, features = ["clock"]`,
// without the `serde` feature, so `DateTime<Utc>` isn't `Serialize` as
// configured. Nothing serializes this record today (`handlers.rs` returns
// bare status codes); add the `serde` feature to the chrono dependency
// first if a future send-pipeline row needs to return these as JSON.
#[derive(Clone, Debug)]
pub struct PushSubscriptionRecord {
    pub handle: String,
    pub endpoint: String,
    pub p256dh: String,
    pub auth: String,
    pub subscribed_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Default)]
pub struct PushStore {
    // Keyed by endpoint -- a browser's PushManager subscription endpoint
    // is already globally unique per (browser, origin) pairing, so it
    // doubles as the natural primary key without generating a uuid for it.
    subscriptions: Mutex<HashMap<String, PushSubscriptionRecord>>,
}

impl PushStore {
    pub fn upsert(&self, record: PushSubscriptionRecord) {
        let mut guard = self.subscriptions.lock().expect("push store lock poisoned");
        guard.insert(record.endpoint.clone(), record);
    }

    pub fn remove(&self, endpoint: &str) {
        let mut guard = self.subscriptions.lock().expect("push store lock poisoned");
        guard.remove(endpoint);
    }

    #[allow(dead_code)] // not consumed until the send pipeline (follow-up row) exists
    pub fn for_handle(&self, handle: &str) -> Vec<PushSubscriptionRecord> {
        let guard = self.subscriptions.lock().expect("push store lock poisoned");
        guard.values().filter(|r| r.handle == handle).cloned().collect()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.subscriptions.lock().expect("push store lock poisoned").len()
    }
}
