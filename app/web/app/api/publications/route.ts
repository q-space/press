import { proxyToApi } from "@/lib/api-client";

/**
 * GET /api/publications -- the creator's own publications, used by the
 * studio's step-1 target picker to resolve an archetype namespace.
 * Publication CRUD itself is Week 2 scope on the engine side; the studio
 * degrades to the general (`_common`) set when this returns nothing.
 */
export async function GET() {
  return proxyToApi("/publications");
}
