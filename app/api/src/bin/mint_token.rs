//! Operator CLI to mint a D-010 thin-client token (BB26091205). There is
//! no login flow yet that could authenticate an HTTP mint *request* (D-010's
//! other two auth paths are Week 2 scope) -- an operator-run CLI is the
//! honest equivalent: whoever can run this already has `JWT_SECRET` (env
//! or Bitwarden), which is the actual authority being exercised.
//!
//! Usage:
//!   JWT_SECRET=... cargo run --bin mint_token -- \
//!     --subject isconl-hub --scope archetypes:read --scope archetypes:preview \
//!     --scope archetypes:generate --ttl-seconds 2592000
//!
//! Prints the signed token to stdout and nothing else, so it's safe to
//! pipe straight into an env file (`... > PRESS_TOKEN.txt`) without a
//! human transcript ever showing the value -- see the relay drive's own
//! secret-handling rule (never print a value you don't have to).
//!
//! `--ttl-seconds` defaults to 30 days (2592000). "Short-lived" per D-010
//! is relative to a browser session (hours), not to an operator's
//! rotation cadence -- there is no automatic refresh path yet (that would
//! need a live minting endpoint, which needs its own auth, which is
//! circular without D-010's other two paths built first), so a token this
//! CLI mints is a manually-rotated credential today. Flagged as follow-up:
//! automate rotation once there's a safe caller to trigger it.

use qspace_press_api::auth::service::issue_thin_client_token;
use std::env;

fn print_usage_and_exit(msg: &str) -> ! {
    eprintln!("error: {msg}");
    eprintln!(
        "usage: mint_token --subject <name> --scope <scope> [--scope <scope> ...] [--ttl-seconds <n>]"
    );
    std::process::exit(2);
}

fn main() {
    let secret = env::var("JWT_SECRET").unwrap_or_default();
    if secret.is_empty() {
        print_usage_and_exit("JWT_SECRET is not set in the environment");
    }

    let args: Vec<String> = env::args().skip(1).collect();
    let mut subject: Option<String> = None;
    let mut scopes: Vec<String> = Vec::new();
    let mut ttl_seconds: usize = 2_592_000; // 30 days

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--subject" => {
                i += 1;
                subject = args.get(i).cloned();
            }
            "--scope" => {
                i += 1;
                if let Some(s) = args.get(i) {
                    scopes.push(s.clone());
                }
            }
            "--ttl-seconds" => {
                i += 1;
                if let Some(s) = args.get(i) {
                    ttl_seconds = s.parse().unwrap_or_else(|_| {
                        print_usage_and_exit("--ttl-seconds must be a positive integer")
                    });
                }
            }
            other => print_usage_and_exit(&format!("unknown argument \"{other}\"")),
        }
        i += 1;
    }

    let subject = subject.unwrap_or_else(|| print_usage_and_exit("--subject is required"));
    if scopes.is_empty() {
        print_usage_and_exit("at least one --scope is required");
    }
    let scope_refs: Vec<&str> = scopes.iter().map(|s| s.as_str()).collect();

    match issue_thin_client_token(&secret, &subject, &scope_refs, ttl_seconds) {
        Ok(token) => println!("{token}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}
