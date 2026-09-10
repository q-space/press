//! Environment config. Read once at boot -- BB26090903's docker-compose.yml
//! supplies these for local dev via a root .env file.

use std::env;

// database_url/redis_url/jwt_secret are read from env here but nothing
// consumes them yet -- db.rs's own connect() (also #[allow(dead_code)]
// for the same reason) isn't called from main.rs's boot sequence until
// Week 2 wires in real entities/migrations. Loading them now, unused, is
// deliberate: it means Config::from_env() already has its final shape,
// so wiring db.rs in later is "call connect(&config.database_url)", not
// a second config-loading pass.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3001),
            database_url: env::var("DATABASE_URL").unwrap_or_default(),
            redis_url: env::var("REDIS_URL").unwrap_or_default(),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_default(),
        }
    }
}
