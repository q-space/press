import { proxyToApi } from "@/lib/api-client";

/** GET /api/archetypes?namespace=_common -- the step-2 picker's list. */
export async function GET(request: Request) {
  const namespace = new URL(request.url).searchParams.get("namespace") ?? "_common";
  return proxyToApi(`/archetypes?namespace=${encodeURIComponent(namespace)}`);
}
