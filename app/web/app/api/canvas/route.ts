import { proxyToApi } from "@/lib/api-client";

/**
 * POST /api/canvas -- the Canvas publish action (BB26091502). Takes
 * `{ title, html }` and nothing else; the engine's own `/canvas` route
 * enforces the same content-type-agnostic shape.
 */
export async function POST(request: Request) {
  return proxyToApi("/canvas", { method: "POST", body: await request.text() });
}
