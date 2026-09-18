// app/web/app/(publication)/[handle]/manifest.webmanifest/route.ts
//
// CHANGELOG:
// - BB26091207: initial per-publication manifest route.
import { NextRequest, NextResponse } from "next/server";

// --- CONFIG BLOCK ---
const BACKGROUND_COLOR = "#0b0f14";
const THEME_COLOR = "#0b0f14";
const ICON_PATHS = {
  standard192: "/icons/icon-192.png",
  standard512: "/icons/icon-512.png",
  maskable512: "/icons/icon-maskable-512.png",
};
// --- END CONFIG BLOCK ---

/**
 * Per-publication Web App Manifest (BB26091207).
 *
 * One static `public/manifest.webmanifest` would give every publication
 * the same `start_url`/`scope`/`name`, so installing from any post lands
 * back on the marketing homepage instead of that publication. Serving it
 * per-`handle` instead means the installed icon takes a reader straight
 * back to the publication they installed it from, and the `scope` is
 * genuinely that publication's own URL space.
 *
 * `name`/icons are placeholders (the handle itself, the shared icon set)
 * until `publications` has a real service backing it (`app/api/src/
 * publications` is still a stub -- see its own `mod.rs`) -- swap in the
 * publication's display name/avatar once that lands. Flagged, not silently
 * left generic.
 */
export async function GET(_req: NextRequest, { params }: { params: { handle: string } }) {
  const { handle } = params;

  const manifest = {
    name: `${handle} — QSpace Press`,
    short_name: handle,
    description: `Read ${handle} on QSpace Press.`,
    id: `/${handle}`,
    start_url: `/${handle}?source=pwa`,
    scope: `/${handle}`,
    display: "standalone",
    display_override: ["standalone", "minimal-ui"],
    background_color: BACKGROUND_COLOR,
    theme_color: THEME_COLOR,
    orientation: "portrait-primary",
    icons: [
      { src: ICON_PATHS.standard192, sizes: "192x192", type: "image/png", purpose: "any" },
      { src: ICON_PATHS.standard512, sizes: "512x512", type: "image/png", purpose: "any" },
      {
        src: ICON_PATHS.maskable512,
        sizes: "512x512",
        type: "image/png",
        purpose: "maskable",
      },
    ],
  };

  return NextResponse.json(manifest, {
    headers: { "Content-Type": "application/manifest+json" },
  });
}
