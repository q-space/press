#!/usr/bin/env node
// app/web/scripts/generate-pwa-icons.mjs
//
// CHANGELOG:
// - BB26091207: initial placeholder icon generator.
/**
 * Generates the reader PWA's placeholder icon set (BB26091207) from a
 * single inline SVG, via `sharp` (already a project dependency for image
 * optimization -- see package.json). Output lands in `public/icons/`,
 * referenced by `[handle]/manifest.webmanifest/route.ts`.
 *
 * These are placeholders: a plain "Q" mark on the brand background color,
 * not real brand art. Swap for a designed icon set before this ships past
 * internal testing -- flagged in the row's own follow-up notes, not
 * silently passed off as final.
 *
 * Usage: node app/web/scripts/generate-pwa-icons.mjs
 */
import sharp from "sharp";
import { mkdir } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const outDir = path.join(__dirname, "..", "public", "icons");

// --- CONFIG BLOCK ---
const BG = "#0b0f14";
const FG = "#ffffff";
// --- END CONFIG BLOCK ---

function markSvg(size, { safeMargin = 0 } = {}) {
  const inner = size - safeMargin * 2;
  const fontSize = Math.round(inner * 0.55);
  return `
    <svg width="${size}" height="${size}" viewBox="0 0 ${size} ${size}" xmlns="http://www.w3.org/2000/svg">
      <rect width="${size}" height="${size}" fill="${BG}" />
      <text x="50%" y="50%" dy=".08em" text-anchor="middle" dominant-baseline="middle"
        font-family="Arial, sans-serif" font-weight="700" font-size="${fontSize}" fill="${FG}">Q</text>
    </svg>`;
}

async function writeIcon(name, size, opts) {
  const svg = Buffer.from(markSvg(size, opts));
  await sharp(svg).png().toFile(path.join(outDir, name));
  console.log(`wrote ${name} (${size}x${size})`);
}

await mkdir(outDir, { recursive: true });
await writeIcon("icon-192.png", 192);
await writeIcon("icon-512.png", 512);
// Maskable icons need a safe zone (~10% margin) since the OS may crop to a
// circle/squircle -- the background still fills the full canvas so nothing
// clips to a hard edge.
await writeIcon("icon-maskable-512.png", 512, { safeMargin: 51 });
