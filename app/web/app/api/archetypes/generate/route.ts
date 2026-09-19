import { proxyToApi } from "@/lib/api-client";

/** POST /api/archetypes/generate -- builds the selected output formats. */
export async function POST(request: Request) {
  return proxyToApi("/archetypes/generate", { method: "POST", body: await request.text() });
}
