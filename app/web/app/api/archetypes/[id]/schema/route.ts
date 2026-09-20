import { canvasFetch, relay, toArchetypeSchema, type CanvasSchema } from "@/lib/canvas";

/**
 * GET /api/archetypes/:id/schema -- the field definitions the studio form is generated from,
 * straight from Canvas. The wizard holds no local copy of these.
 */
export async function GET(_request: Request, { params }: { params: { id: string } }) {
  const res = await canvasFetch(`/v1/archetypes/${encodeURIComponent(params.id)}/schema`);
  if (!res.ok) return relay(res);
  return Response.json(toArchetypeSchema((await res.json()) as CanvasSchema));
}
