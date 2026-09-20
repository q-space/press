import { errorResponse, listSchemas } from "@/lib/canvas";

/** GET /api/archetypes?namespace=_common -- the step-2 picker's list, from Canvas. */
export async function GET(request: Request) {
  const namespace = new URL(request.url).searchParams.get("namespace") ?? "_common";
  // Only the shared archetypes exist; per-engagement namespaces were never populated.
  if (namespace !== "_common") return errorResponse(404, `no archetype namespace "${namespace}" -- known namespaces: _common`);
  const schemas = await listSchemas();
  if (schemas instanceof Response) return schemas;
  return Response.json({
    archetypes: schemas.map((s) => ({ id: s.id, title: s.title, governance: !!s.governance, layout: s.layout ?? null })),
  });
}
