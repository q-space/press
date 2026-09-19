//! HTTP handlers for structured list documents. Not yet built -- needs
//! `db.rs` connected and SeaORM entities generated from
//! `migrations/004_structured_lists.sql` first, same stage as
//! `payments/handlers.rs`. The DB-backed PATCH handler is the same
//! compare-and-swap shape as `service::apply_patch`
//! (`UPDATE structured_list_items SET ... WHERE id = $1 AND version = $2`,
//! `Conflict` when the update affects zero rows).
