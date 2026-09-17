import { proxyToApi } from "@/lib/api-client";

/**
 * Studio draft autosave, persisted server-side.
 *
 * Deliberately NOT localStorage: iSconl's wizard autosaves to the
 * browser, which loses a half-written document the moment the creator
 * changes machine, and BB26091204 excludes that from what gets copied
 * across. The draft belongs to the account, so it lives with the account.
 *
 * `key` identifies one draft as target + namespace + archetype, which is
 * what the wizard can reconstruct on the way back in.
 */
export async function GET(request: Request) {
  const key = new URL(request.url).searchParams.get("key");
  if (!key) return Response.json({ error: "key is required" }, { status: 400 });
  return proxyToApi(`/documents/drafts/${encodeURIComponent(key)}`);
}

export async function PUT(request: Request) {
  return proxyToApi("/documents/drafts", { method: "PUT", body: await request.text() });
}
