import { canvasFetch, errorResponse, relay } from "@/lib/canvas";

/**
 * POST /api/archetypes/preview -- renders the live preview through Canvas's HTML renderer,
 * the same one the export uses, so the preview is the output rather than an approximation.
 */
export async function POST(request: Request) {
  const { archetypeId, content } = (await request.json().catch(() => ({}))) as { archetypeId?: string; content?: unknown };
  if (!archetypeId) return errorResponse(400, "archetypeId is required");
  const res = await canvasFetch("/v1/render", { method: "POST", body: { archetype: archetypeId, content: content ?? {}, format: "html" } });
  if (!res.ok) return relay(res, Object.keys((content as object) ?? {}));
  return Response.json({ html: await res.text() });
}
