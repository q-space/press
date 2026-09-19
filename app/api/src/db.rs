//! Database pool. Wired into `main.rs`'s boot sequence as of BB26091502
//! (Canvas is the first caller that needs real persistence) -- best-effort:
//! a missing or unreachable `DATABASE_URL` degrades whatever depends on it
//! to a 503 rather than blocking boot, since most of this API still has
//! nothing to connect for (`posts`/`publications` are still CRUD stubs).

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}
