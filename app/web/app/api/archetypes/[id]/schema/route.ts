import { proxyToApi } from "@/lib/api-client";

/**
 * GET /api/archetypes/:id/schema -- the field definitions the studio form
 * is generated from. The wizard holds no local copy of these.
 */
export async function GET(_request: Request, { params }: { params: { id: string } }) {
  return proxyToApi(`/archetypes/${encodeURIComponent(params.id)}/schema`);
}
