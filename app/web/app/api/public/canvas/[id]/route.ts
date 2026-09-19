import { proxyToApi } from "@/lib/api-client";

/**
 * GET /api/public/canvas/:id -- Canvas's unauthenticated share route
 * (BB26091502). Generic sibling of iSconl's
 * `/api/public/learn/:course/:slug` (built for standalone module export):
 * no login, no account, just the id. Serves the artifact's own HTML
 * straight through (the engine returns `text/html`, not JSON, for this
 * one), so the link is directly viewable/printable/embeddable.
 */
export async function GET(_request: Request, { params }: { params: { id: string } }) {
  return proxyToApi(`/public/canvas/${encodeURIComponent(params.id)}`);
}
