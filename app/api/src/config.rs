//! Environment config. Read once at boot -- BB26090903's docker-compose.yml
//! supplies these for local dev via a root .env file.

use std::env;

// database_url is now consumed at boot (main.rs, BB26091502) to wire
// db.rs's connection pool in for Canvas. redis_url/jwt_secret are still
// read but unused -- kept here so Config::from_env() has its final shape
// and wiring either in later is a one-line change, not a second
// config-loading pass.
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
