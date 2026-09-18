//! Axum extractor enforcing canon D-010's thin-client auth path on
//! `/archetypes/*` (BB26091205). Verifies signature, audience and expiry
//! (`auth::service`); per-route scope is checked by the handler via
//! `require_scope`, since which scope a route needs is a property of the
//! route, not something the extractor itself knows.
//!
//! This is NOT the browser-cookie or native-bearer-token middleware D-010
//! also specifies -- those are Week 2 scope, alongside `auth/handlers.rs`.

use crate::auth::service::{verify_thin_client_token, ThinClientClaims};
use crate::config::Config;
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// Extract with `ThinClientAuth(claims)` in any handler mounted on a
/// router whose state is `Config` (see `generate::handlers::router`).
pub struct ThinClientAuth(pub ThinClientClaims);

pub struct AuthRejection(pub StatusCode, pub &'static str);

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        (self.0, Json(json!({ "error": self.1 }))).into_response()
    }
}

impl FromRequestParts<Config> for ThinClientAuth {
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, state: &Config) -> Result<Self, Self::Rejection> {
        let raw = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthRejection(StatusCode::UNAUTHORIZED, "missing bearer token"))?;
        let token = raw
            .strip_prefix("Bearer ")
            .ok_or(AuthRejection(StatusCode::UNAUTHORIZED, "malformed authorization header"))?;
        let claims = verify_thin_client_token(&state.jwt_secret, token)
            .map_err(|_| AuthRejection(StatusCode::UNAUTHORIZED, "invalid or expired token"))?;
        Ok(ThinClientAuth(claims))
    }
}

/// Called by each handler with the scope its route requires. A separate
/// step from the extractor (rather than a generic-per-scope extractor) so
/// the 403-vs-401 distinction (authenticated but not permitted, vs not
/// authenticated at all) stays visible per handler.
pub fn require_scope(
    claims: &ThinClientClaims,
    required: &str,
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if claims.scope.iter().any(|s| s == required) {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            Json(json!({ "error": format!("token missing required scope \"{required}\"") })),
        ))
    }
}
