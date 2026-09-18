// app/web/app/(publication)/[handle]/layout.tsx
//
// CHANGELOG:
// - BB26091207: initial reader-route-group layout, wires the per-handle
//   manifest + PwaShell in without touching the root layout.
import type { Metadata, Viewport } from "next";
import PwaShell from "@/components/pwa/PwaShell";

// --- CONFIG BLOCK ---
const THEME_COLOR = "#0b0f14";
// --- END CONFIG BLOCK ---

/**
 * Reader route group layout (BB26091207) -- wraps ONLY `/[handle]` and
 * `/[handle]/[slug]`, the actual reader surface per canon D-011. Creator
 * Studio (`(creator)`), auth (`(auth)`) and marketing (`(marketing)`) each
 * have their own route groups and never render this layout, so they never
 * get the manifest link, never mount `PwaShell`, and never register the
 * service worker.
 *
 * `generateMetadata` (not a static `<link>`) is what wires the per-handle
 * manifest in -- Next's Metadata API renders it into `<head>` for us, and
 * per-handle means installing from any publication's post takes that
 * reader back to that publication, not the marketing homepage. See
 * `[handle]/manifest.webmanifest/route.ts` for why this is per-handle at
 * all.
 */
export async function generateMetadata({
  params,
}: {
  params: { handle: string };
}): Promise<Metadata> {
  return {
    manifest: `/${params.handle}/manifest.webmanifest`,
  };
}

export const viewport: Viewport = {
  themeColor: THEME_COLOR,
};

export default function ReaderLayout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: { handle: string };
}) {
  return (
    <>
      {children}
      <PwaShell handle={params.handle} />
    </>
  );
}
