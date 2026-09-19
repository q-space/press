/**
 * BB26091208 part 1/verification: HTML -> PDF via `Page.printToPDF` over
 * the Chrome DevTools Protocol, through `puppeteer-core`'s `page.pdf()`
 * (which is a thin wrapper over exactly that CDP call -- confirmed
 * against puppeteer-core's own source before writing this, not assumed).
 *
 * Deliberately NOT the Chrome CLI (`--headless --print-to-pdf`): Chrome
 * 151+ dropped `--print-to-pdf-no-header`, so a CLI print bakes in
 * Chrome's own default header/footer with no flag left to suppress it.
 * `page.pdf({ displayHeaderFooter: false })` sets that field on the CDP
 * call directly, unaffected by the CLI regression.
 *
 * `preferCSSPageSize: true` is the other load-bearing option here: it's
 * what makes the PDF obey render_html.rs's own `@page { size: A4; margin:
 * ...mm }` rule (the "margins come from @page, never from container
 * padding" pattern the reference template and render_html.rs's own tests
 * both enforce) instead of silently falling back to a default Letter/no-
 * margin page.
 *
 * Usage: node scripts/render-pdf-cdp.mjs <input.html> <output.pdf>
 * Env: CHROME_PATH overrides the auto-detected Chrome executable.
 */
import { readFile, access } from "node:fs/promises";
import { resolve } from "node:path";
import puppeteer from "puppeteer-core";

function defaultChromePath() {
  if (process.env.CHROME_PATH) return process.env.CHROME_PATH;
  const candidates =
    process.platform === "win32"
      ? [
          "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
          "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
        ]
      : process.platform === "darwin"
        ? ["/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"]
        : ["/usr/bin/google-chrome", "/usr/bin/chromium-browser", "/usr/bin/chromium"];
  return candidates[0];
}

async function findChromePath() {
  const preferred = defaultChromePath();
  try {
    await access(preferred);
    return preferred;
  } catch {
    throw new Error(
      `no Chrome executable found at "${preferred}" -- set CHROME_PATH to override`,
    );
  }
}

/**
 * `html` -> PDF bytes (`Buffer`). Exported so a caller (a test, or a
 * future generate-route handler) can use this without shelling out to
 * the CLI form below.
 */
export async function renderPdfViaCdp(html, { chromePath } = {}) {
  const executablePath = chromePath || (await findChromePath());
  const browser = await puppeteer.launch({
    executablePath,
    headless: true,
    args: ["--no-sandbox", "--disable-gpu"],
  });
  try {
    const page = await browser.newPage();
    // `networkidle0`: the page has inline CSS only (no external asset/
    // network dependency, per render_html.rs's own doc comment), so this
    // just waits for layout to settle, not for any real network activity.
    await page.setContent(html, { waitUntil: "networkidle0" });
    const pdf = await page.pdf({
      // The CDP field this whole module exists to set explicitly, since
      // the Chrome CLI can no longer set it at all (see module doc comment).
      displayHeaderFooter: false,
      printBackground: true,
      // Obeys render_html.rs's own `@page { size: A4; margin: ... }`
      // instead of a hardcoded Node-side page size/margin.
      preferCSSPageSize: true,
    });
    return Buffer.from(pdf);
  } finally {
    await browser.close();
  }
}

async function main() {
  const [, , inputPath, outputPath] = process.argv;
  if (!inputPath || !outputPath) {
    console.error("usage: node scripts/render-pdf-cdp.mjs <input.html> <output.pdf>");
    process.exit(1);
  }
  const html = await readFile(resolve(inputPath), "utf8");
  const pdf = await renderPdfViaCdp(html);
  const { writeFile } = await import("node:fs/promises");
  await writeFile(resolve(outputPath), pdf);
  console.log(`wrote ${outputPath} (${pdf.length} bytes)`);
}

// Only run as a CLI when invoked directly (`node scripts/render-pdf-cdp.mjs`),
// not when imported (e.g. from a future test) -- same "importable module
// with a CLI main()" shape `mock-engine.mjs` doesn't need but other repos'
// scripts in this session's context (status-brief.js) already follow.
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch((e) => {
    console.error(e);
    process.exit(1);
  });
}
