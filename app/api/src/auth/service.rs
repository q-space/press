//! Thin-client token issuance + verification -- canon D-010's third auth
//! path (`work/_arc/press/canon-canvas/20260915_project_development_
//! canonical_qspace_press_v2_1_0.md`, Decision Log D-010 and "Three
//! Authentication Paths, Not One"). Scoped (an explicit `scope` claim,
//! checked per-route by the caller via `has_scope`/`require_scope`),
//! short-lived (`exp`, caller-supplied TTL), audience-restricted (`aud`
//! must equal `THIN_CLIENT_AUDIENCE`, so this token can never be replayed
//! as a browser-session or native-companion token even though all three
//! paths share `JWT_SECRET` today -- see the module-level note on why
//! that sharing is a known gap, not an oversight).
//!
//! Deliberately NOT the browser-cookie or native-bearer-token paths (the
//! other two legs of D-010) -- those are Week 2 scope (`auth/handlers.rs`,
//! `auth/middleware.rs`'s own header comment). This module exists now
//! because BB26091205 (the thin-client `/archetypes/*` API) is the first
//! consumer of any auth path at all.
//!
//! **Known gap, flagged rather than fixed here (out of this row's
//! scope):** all three D-010 paths would sign against the same
//! `JWT_SECRET` once the other two are built, since `Config` only carries
//! one secret today. A leaked thin-client token could not forge a browser
//! session (different claim shape, and `aud` still restricts it), but the
//! signing key itself is shared blast radius. Splitting into a dedicated
//! `THIN_CLIENT_JWT_SECRET` is cheap once there's a second consumer to
//! justify the extra env var -- tracked as follow-up, not blocking this
//! row's first consumer.

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// The one audience value every thin-client token is restricted to. A
/// token minted for this audience is rejected by any verifier checking a
/// different audience. Only one exists today, but the claim is present
/// from day one so a second (e.g. a distinct audience per calling
/// service, rather than one shared by both the web BFF and iSconl hub) is
/// a matter of parameterizing this, not retrofitting the claim shape.
pub const THIN_CLIENT_AUDIENCE: &str = "qspace-press-thin-client";

pub const SCOPE_ARCHETYPES_READ: &str = "archetypes:read";
pub const SCOPE_ARCHETYPES_PREVIEW: &str = "archetypes:preview";
pub const SCOPE_ARCHETYPES_GENERATE: &str = "archetypes:generate";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinClientClaims {
    /// Who this token was minted for -- an identifier for the calling
    /// *service* (e.g. "isconl-hub", "qspace-web-bff"), never a creator
    /// id. This is what "must not carry a creator's full session" means
    /// structurally: there is no field here a session could be smuggled
    /// into, not just a policy saying not to put one there.
    pub sub: String,
    pub aud: String,
    pub scope: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug)]
pub struct TokenError(pub String);

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for TokenError {}

fn now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as usize
}

/// `ttl_seconds` is caller-supplied rather than a fixed constant so
/// `bin/mint_token.rs` (an operator-run CLI, not yet an HTTP endpoint --
/// no login flow exists yet to authenticate a mint *request*) can choose
/// a TTL that fits its own rotation cadence per consumer.
pub fn issue_thin_client_token(
    secret: &str,
    subject: &str,
    scope: &[&str],
    ttl_seconds: usize,
) -> Result<String, TokenError> {
    let iat = now();
    let claims = ThinClientClaims {
        sub: subject.to_string(),
        aud: THIN_CLIENT_AUDIENCE.to_string(),
        scope: scope.iter().map(|s| s.to_string()).collect(),
        exp: iat + ttl_seconds,
        iat,
    };
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| TokenError(format!("failed to sign thin-client token: {e}")))
}

/// Verifies signature, expiry (zero leeway -- short-lived means short-
/// lived) and audience. Scope is NOT checked here -- see `has_scope`,
/// called per-route by the handler, since which scope a route needs is a
/// property of the route, not of the token itself.
pub fn verify_thin_client_token(secret: &str, token: &str) -> Result<ThinClientClaims, TokenError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[THIN_CLIENT_AUDIENCE]);
    validation.leeway = 0;
    let data = decode::<ThinClientClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map_err(|e| TokenError(format!("invalid thin-client token: {e}")))?;
    Ok(data.claims)
}

pub fn has_scope(claims: &ThinClientClaims, required: &str) -> bool {
    claims.scope.iter().any(|s| s == required)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_valid_token() {
        let token =
            issue_thin_client_token("test-secret", "isconl-hub", &[SCOPE_ARCHETYPES_READ], 60).unwrap();
        let claims = verify_thin_client_token("test-secret", &token).unwrap();
        assert_eq!(claims.sub, "isconl-hub");
        assert_eq!(claims.aud, THIN_CLIENT_AUDIENCE);
        assert!(has_scope(&claims, SCOPE_ARCHETYPES_READ));
        assert!(!has_scope(&claims, SCOPE_ARCHETYPES_GENERATE));
    }

    #[test]
    fn rejects_a_token_signed_with_a_different_secret() {
        let token =
            issue_thin_client_token("secret-a", "isconl-hub", &[SCOPE_ARCHETYPES_READ], 60).unwrap();
        assert!(verify_thin_client_token("secret-b", &token).is_err());
    }

    #[test]
    fn rejects_an_expired_token() {
        let token = issue_thin_client_token("test-secret", "isconl-hub", &[SCOPE_ARCHETYPES_READ], 0).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(verify_thin_client_token("test-secret", &token).is_err());
    }

    #[test]
    fn rejects_a_token_for_a_different_audience() {
        // Simulates a future second D-010 path (browser/native) minting
        // against a different audience -- must not verify here even with
        // the right secret, since that's the whole point of the claim.
        let iat = now();
        let claims = ThinClientClaims {
            sub: "someone".into(),
            aud: "some-other-audience".into(),
            scope: vec![SCOPE_ARCHETYPES_READ.into()],
            exp: iat + 60,
            iat,
        };
        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(b"test-secret"),
        )
        .unwrap();
        assert!(verify_thin_client_token("test-secret", &token).is_err());
    }
}
