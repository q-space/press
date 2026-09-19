import { proxyToApi } from "@/lib/api-client";

/**
 * GET/PUT /api/canvas/:id -- the live-editable side of Canvas
 * (BB26091502). PUT is what the editor's autosave calls on every
 * debounced edit; the artifact stays editable after publish rather than
 * being a frozen render, which is the whole point of the name.
 */
export async function GET(_request: Request, { params }: { params: { id: string } }) {
  return proxyToApi(`/canvas/${encodeURIComponent(params.id)}`);
}

export async function PUT(request: Request, { params }: { params: { id: string } }) {
  return proxyToApi(`/canvas/${encodeURIComponent(params.id)}`, {
    method: "PUT",
    body: await request.text(),
  });
}
