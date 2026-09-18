// app/web/app/api/push/unsubscribe/route.ts
//
// CHANGELOG:
// - BB26091207: initial push-unsubscribe BFF proxy route.
import { proxyToApi } from "@/lib/api-client";

// --- CONFIG BLOCK ---
const UPSTREAM_PATH = "/push/unsubscribe";
// --- END CONFIG BLOCK ---

/** POST /api/push/unsubscribe (BB26091207) -- forwards to the Rust engine. */
export async function POST(request: Request) {
  const body = await request.text();
  return proxyToApi(UPSTREAM_PATH, { method: "POST", body });
}
