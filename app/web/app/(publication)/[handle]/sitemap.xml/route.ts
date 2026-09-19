import { NextRequest, NextResponse } from "next/server";

/** Per-publication sitemap — not yet built; returns an empty, valid sitemap shell. */
export async function GET(_req: NextRequest, { params }: { params: { handle: string } }) {
  const body = `<?xml version="1.0" encoding="UTF-8"?>\n<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9"></urlset>`;
  return new NextResponse(body, { headers: { "Content-Type": "application/xml" } });
}
