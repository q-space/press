/**
 * Server-side client for Canvas (canvas.acexoft.com), the document engine.
 *
 * Qpress used to carry the generation engine itself (`app/api/src/generate/`, deleted in
 * BP26091907). It now keeps only what is about publishing and asks Canvas for documents:
 * the Studio's archetype list, each archetype's field schema, the live preview and the
 * exported files all come from Canvas over HTTP.
 *
 * Auth is a RENDER-ONLY token (CANVAS_RENDER_TOKEN, server-side only, never sent to the
 * browser): it can render and validate and nothing else, so a leak means "someone can render
 * documents", never "someone can read a creator's stored work". Press sends Canvas a resolved
 * payload (archetype + content) and gets an artifact back; it never sends a publication id,
 * post, or subscriber, because Canvas must not learn those concepts.
 *
 * Kept free of `@/` imports so it can be unit-tested with plain `node --test`.
 */

export interface CanvasField {
  name: string;
  label: string;
  type: string;
  required: boolean;
  section?: string | null;
  keys?: string[] | null;
}

export interface CanvasSchema {
  id: string;
  title: string;
  governance?: boolean;
  layout?: string | null;
  filename_fields?: [string, string] | null;
  fields: CanvasField[];
}

export interface FieldError {
  field: string;
  message: string;
}

const SCHEMA_TTL_MS = 5 * 60 * 1000;
let cache: { at: number; schemas: CanvasSchema[] } | null = null;

export function canvasBase(): string {
  return (process.env.CANVAS_URL || "https://canvas.acexoft.com").replace(/\/+$/, "");
}

/** A JSON error Response, in the shape the wizard already reads (`{error, errors?}`). */
export function errorResponse(status: number, error: string, errors?: FieldError[]): Response {
  return Response.json(errors && errors.length ? { error, errors } : { error }, { status });
}

/** Fetch from Canvas with the render-only token. Returns a Response (never throws). */
export async function canvasFetch(path: string, init: { method?: string; body?: unknown } = {}): Promise<Response> {
  const token = process.env.CANVAS_RENDER_TOKEN;
  if (!token) return errorResponse(503, "CANVAS_RENDER_TOKEN is not set -- the document engine is unreachable.");
  try {
    return await fetch(`${canvasBase()}${path}`, {
      method: init.method ?? "GET",
      headers: { Authorization: `Bearer ${token}`, ...(init.body ? { "Content-Type": "application/json" } : {}) },
      body: init.body ? JSON.stringify(init.body) : undefined,
      signal: AbortSignal.timeout(30_000),
      cache: "no-store",
    });
  } catch (cause) {
    const why = cause instanceof Error && cause.name === "TimeoutError" ? "no answer within 30s" : cause instanceof Error ? cause.message : String(cause);
    return errorResponse(502, `could not reach the document engine: ${why}`);
  }
}

/**
 * Canvas reports a content problem as one string, e.g.
 * "single-page-memo: missing required field: to; missing required field: from".
 * The wizard binds errors to controls, so split that back into per-field errors.
 */
export function toFieldErrors(message: string, knownFields: string[] = []): FieldError[] {
  const out: FieldError[] = [];
  const re = /missing required field:\s*([A-Za-z0-9_.-]+)/g;
  for (let m = re.exec(message); m; m = re.exec(message)) {
    out.push({ field: m[1], message: `${m[1].replace(/_/g, " ")} is required` });
  }
  if (!out.length) {
    for (const seg of message.split(";")) {
      const name = knownFields.find((f) => new RegExp(`\\b${f}\\b`).test(seg));
      if (name) out.push({ field: name, message: seg.replace(/^[^:]*:\s*/, "").trim() || seg.trim() });
    }
  }
  return out;
}

/** Canvas schema -> the wizard's `ArchetypeSchema` (lib/archetypes.ts). */
export function toArchetypeSchema(s: CanvasSchema) {
  const [primary, secondary] = s.filename_fields ?? ["", ""];
  return {
    id: s.id,
    title: s.title,
    governance: !!s.governance,
    layout: s.layout ?? null,
    filenameFields: { primary, secondary },
    fields: s.fields.map((f) => ({
      name: f.name,
      label: f.label,
      type: f.type,
      required: !!f.required,
      ...(f.section ? { section: f.section } : {}),
      ...(f.keys ? { keys: f.keys } : {}),
    })),
  };
}

/** Every archetype's schema, cached briefly (a handful of small documents that rarely change). */
export async function listSchemas(now: () => number = Date.now): Promise<CanvasSchema[] | Response> {
  if (cache && now() - cache.at < SCHEMA_TTL_MS) return cache.schemas;
  const list = await canvasFetch("/v1/archetypes");
  if (!list.ok) return relay(list);
  const { archetypes } = (await list.json()) as { archetypes: Array<{ id: string }> };
  const schemas: CanvasSchema[] = [];
  for (const a of archetypes) {
    const r = await canvasFetch(`/v1/archetypes/${encodeURIComponent(a.id)}/schema`);
    if (!r.ok) return relay(r);
    schemas.push((await r.json()) as CanvasSchema);
  }
  cache = { at: now(), schemas };
  return schemas;
}

/** Pass a failed Canvas response through as the wizard's error shape. */
export async function relay(res: Response, knownFields: string[] = []): Promise<Response> {
  let message = `${res.status} ${res.statusText}`;
  try {
    const body = (await res.clone().json()) as { error?: string };
    if (body?.error) message = body.error;
  } catch {
    // keep the status line
  }
  return errorResponse(res.status, message, toFieldErrors(message, knownFields));
}

/** Test seam: forget the schema cache. */
export function resetCache() {
  cache = null;
}
