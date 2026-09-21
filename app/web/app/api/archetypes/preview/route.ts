import { proxyToApi } from "@/lib/api-client";

/**
 * POST /api/archetypes/preview -- renders the live preview through the
 * SAME html renderer the export uses, so the preview is the output rather
 * than an approximation of it (this row's standing rule).
 */
export async function POST(request: Request) {
  return proxyToApi("/archetypes/preview", { method: "POST", body: await request.text() });
}
