/**
 * Canvas (BB26091502): the studio's publish action. Content-type
 * agnostic by design -- a Canvas is a title and rendered HTML, full stop.
 * This file must never grow a field that presumes what produced that
 * HTML (a course module, an archetype document, anything else); see the
 * row's own note on why (iSpark stays a separate product from Qpress).
 */

export interface CanvasArtifact {
  id: string;
  title: string;
  content_html: string;
  created_at: string;
  updated_at: string;
}

export interface CreateCanvasInput {
  title: string;
  html: string;
}

export interface UpdateCanvasInput {
  title?: string;
  html?: string;
}

async function readError(res: Response): Promise<string> {
  try {
    const body = (await res.json()) as { error?: string };
    if (body?.error) return body.error;
  } catch {
    // fall through to the status line
  }
  return `${res.status} ${res.statusText}`;
}

export async function createCanvas(input: CreateCanvasInput): Promise<CanvasArtifact> {
  const res = await fetch("/api/canvas", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(input),
  });
  if (!res.ok) throw new Error(await readError(res));
  return res.json() as Promise<CanvasArtifact>;
}

export async function getCanvas(id: string): Promise<CanvasArtifact> {
  const res = await fetch(`/api/canvas/${encodeURIComponent(id)}`, { cache: "no-store" });
  if (!res.ok) throw new Error(await readError(res));
  return res.json() as Promise<CanvasArtifact>;
}

export async function updateCanvas(id: string, input: UpdateCanvasInput): Promise<CanvasArtifact> {
  const res = await fetch(`/api/canvas/${encodeURIComponent(id)}`, {
    method: "PUT",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(input),
  });
  if (!res.ok) throw new Error(await readError(res));
  return res.json() as Promise<CanvasArtifact>;
}

/** The unauthenticated, no-login share link -- same shape as this
 * project's `/api/public/learn/:course/:slug` precedent, generalised. */
export function publicCanvasUrl(id: string): string {
  return `/api/public/canvas/${encodeURIComponent(id)}`;
}
