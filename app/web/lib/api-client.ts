/**
 * Typed fetch wrapper for the Rust API (app/api). Not yet built out --
 * base URL comes from QSPACE_API_URL (set in .env, see BB26090903's
 * docker-compose.yml for the local-dev value).
 */
const API_BASE_URL = process.env.QSPACE_API_URL ?? "";

export async function apiFetch<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${API_BASE_URL}${path}`, init);
  if (!res.ok) throw new Error(`API request failed: ${res.status} ${res.statusText}`);
  return res.json() as Promise<T>;
}
