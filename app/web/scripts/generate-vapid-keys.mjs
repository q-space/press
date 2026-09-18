#!/usr/bin/env node
// app/web/scripts/generate-vapid-keys.mjs
//
// CHANGELOG:
// - BB26091207: initial VAPID keypair generator (Node WebCrypto, no
//   external dependency).
/**
 * Generates a VAPID (application server) EC P-256 key pair for web push
 * (BB26091207), using Node's built-in WebCrypto -- no `web-push` npm
 * dependency needed just to run this once per environment.
 *
 * Usage: node app/web/scripts/generate-vapid-keys.mjs
 * Then copy the printed values into .env:
 *   NEXT_PUBLIC_VAPID_PUBLIC_KEY=<public>
 *   VAPID_PRIVATE_KEY=<private>
 *
 * Output format matches what the `web-push` library (and most other
 * implementations) expect: the public key is the 65-byte uncompressed EC
 * point, the private key is the raw 32-byte scalar ("d") -- both
 * base64url, no padding.
 */
import { webcrypto } from "node:crypto";

function toBase64Url(buffer) {
  return Buffer.from(buffer)
    .toString("base64")
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
}

const { publicKey, privateKey } = await webcrypto.subtle.generateKey(
  { name: "ECDSA", namedCurve: "P-256" },
  true,
  ["sign", "verify"],
);

const rawPublic = await webcrypto.subtle.exportKey("raw", publicKey);
const jwkPrivate = await webcrypto.subtle.exportKey("jwk", privateKey);

console.log("NEXT_PUBLIC_VAPID_PUBLIC_KEY=" + toBase64Url(rawPublic));
console.log("VAPID_PRIVATE_KEY=" + jwkPrivate.d);
