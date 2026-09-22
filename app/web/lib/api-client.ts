/**
 * Typed fetch wrapper for the Rust API (app/api). Base URL comes from
 * QSPACE_API_URL (set in .env, see BB26090903's docker-compose.yml for the
 * local-dev value).
 */
const API_BASE_URL = process.env.QSPACE_API_URL ?? "";

export async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE_URL}${path}`, init);
  if (!res.ok) throw new Error(`API request failed: ${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}

/**
 * Server-side proxy used by the route handlers under `app/api/`.
 *
 * The browser never talks to the Rust API directly: the creator's session
 * stays on the Next server, and what crosses to the engine is a scoped,
 * short-lived, audience-restricted token per canon D-010 (BB26091205 owns
 * minting it -- until then this forwards whatever QSPACE_API_TOKEN holds,
 * and sends nothing if it is unset rather than leaking a session cookie).
 *
 * The upstream body and status are passed through untouched, so a
 * per-field validation error arrives at the wizard as the engine wrote it
 * instead of being flattened into one opaque failure.
 */
export async function proxyToApi(
  path: string,
  init: { method?: string; body?: string } = {},
): Promise<Response> {
  if (!API_BASE_URL) {
    return Response.json(
      { error: "QSPACE_API_URL is not set -- the generation engine is unreachable." },
      { status: 503 },
    );
  }
  const headers: Record<string, string> = { "Content-Type": "application/json" };
  const token = process.env.QSPACE_API_TOKEN;
  if (token) headers.Authorization = `Bearer ${token}`;

  let upstream: Response;
  try {
    upstream = await fetch(`${API_BASE_URL}${path}`, {
      method: init.method ?? "GET",
      headers,
      body: init.body,
      cache: "no-store",
    });
  } catch (cause) {
    return Response.json(
      { error: `Could not reach the generation engine at ${API_BASE_URL}: ${String(cause)}` },
      { status: 502 },
    );
  }

  const text = await upstream.text();
  return new Response(text, {
    status: upstream.status,
    headers: { "Content-Type": upstream.headers.get("Content-Type") ?? "application/json" },
  });
}
