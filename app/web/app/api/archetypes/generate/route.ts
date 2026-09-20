import { canvasFetch, errorResponse, relay } from "@/lib/canvas";

const FORMATS = ["md", "docx", "pdf", "html"];

/**
 * POST /api/archetypes/generate -- builds the selected output formats through Canvas, one
 * call per format. Canvas names each file by the naming convention and returns it in a header.
 */
export async function POST(request: Request) {
  const { archetypeId, formats, content } = (await request.json().catch(() => ({}))) as {
    archetypeId?: string;
    formats?: string[];
    content?: unknown;
  };
  if (!archetypeId) return errorResponse(400, "archetypeId is required");
  const wanted = (formats ?? []).filter((f) => FORMATS.includes(f));
  if (!wanted.length) return errorResponse(400, "pick at least one output format (md, docx, pdf, html)");

  const files: Record<string, { filename: string; bytes: number; base64: string }> = {};
  for (const format of wanted) {
    const res = await canvasFetch("/v1/generate", { method: "POST", body: { archetype: archetypeId, content: content ?? {}, format } });
    if (!res.ok) return relay(res, Object.keys((content as object) ?? {}));
    const filename = res.headers.get("x-canvas-filename");
    if (!filename) return errorResponse(502, "the document engine did not return a filename");
    const bytes = Buffer.from(await res.arrayBuffer());
    files[format] = { filename, bytes: bytes.length, base64: bytes.toString("base64") };
  }
  return Response.json({ archetypeId, files });
}
