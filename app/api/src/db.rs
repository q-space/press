//! Database pool. Not yet wired into `main.rs`'s boot sequence -- there are
//! no entities/migrations to connect for yet (Week 2 work, once
//! `migrations/001_initial_schema.sql` lands and SeaORM entities are
//! generated from it). Kept here, ready to call, so wiring it in later is
//! a one-line change in `main.rs`, not a new module.

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
    Database::connect(database_url).await
}
