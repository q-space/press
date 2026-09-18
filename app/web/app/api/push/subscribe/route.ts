// app/web/app/api/push/subscribe/route.ts
//
// CHANGELOG:
// - BB26091207: initial push-subscribe BFF proxy route.
import { proxyToApi } from "@/lib/api-client";

// --- CONFIG BLOCK ---
const UPSTREAM_PATH = "/push/subscribe";
// --- END CONFIG BLOCK ---

/**
 * POST /api/push/subscribe (BB26091207). Browser-facing BFF route -- the
 * browser never talks to `app/api` directly (same pattern as every other
 * route under `app/api/` here), it forwards to the Rust engine's
 * `/push/subscribe` (`app/api/src/push`).
 */
export async function POST(request: Request) {
  const body = await request.text();
  return proxyToApi(UPSTREAM_PATH, { method: "POST", body });
}
