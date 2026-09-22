/**
 * A stand-in for the generation engine, so Creator Studio can be driven
 * end to end before BB26091205 wires the real routes.
 *
 * It answers exactly the four endpoints the studio calls, in the shape
 * BB26091205 specifies, plus `/publications` and the draft store. It is a
 * fixture for developing and eyeballing the wizard -- not a second
 * implementation of anything. The archetype it serves is hand-written
 * here and deliberately exercises every field type in the vocabulary
 * (`text`, `textarea`, `select`, `list`, `reasoned-list`, `table-list`)
 * plus `section` grouping, which no single real archetype does.
 *
 *   node scripts/mock-engine.mjs            # listens on 3001
 *   QSPACE_API_URL=http://localhost:3001 pnpm dev
 *
 * Zero dependencies on purpose: a fixture that needs installing is a
 * fixture nobody runs.
 */
import { createServer } from "node:http";

const PORT = Number(process.env.PORT ?? 3001);

const ARCHETYPES = [
  {
    id: "single-page-memo",
    title: "Single-Page Memo",
    governance: false,
    layout: null,
    filenameFields: { primary: "re", secondary: "date" },
    fields: [
      { name: "to", label: "To", type: "text", required: true },
      { name: "from", label: "From", type: "text", required: true },
      { name: "date", label: "Date", type: "text", required: true },
      { name: "re", label: "Re", type: "text", required: true },
      { name: "summary", label: "Summary", type: "textarea", required: true },
      { name: "points", label: "Key points", type: "list", required: false },
    ],
  },
  {
    id: "fixture-every-field-type",
    title: "Fixture: every field type",
    governance: true,
    layout: null,
    filenameFields: { primary: "subject", secondary: "date" },
    fields: [
      { name: "subject", label: "Subject", type: "text", required: true, section: "Header" },
      { name: "date", label: "Date", type: "text", required: true, section: "Header" },
      {
        name: "status",
        label: "Status",
        type: "select",
        required: true,
        section: "Header",
        options: ["Draft", "For review", "Final"],
      },
      { name: "narrative", label: "Narrative", type: "textarea", required: true, section: "Body" },
      { name: "findings", label: "Findings", type: "list", required: false, section: "Body" },
      {
        name: "options",
        label: "Options considered",
        type: "reasoned-list",
        required: false,
        section: "Body",
        keys: ["option", "detail"],
      },
      {
        name: "inventory",
        label: "Inventory",
        type: "table-list",
        required: false,
        section: "Appendix",
        keys: ["category", "field", "example_value", "notes"],
      },
    ],
  },
];

const drafts = new Map();

function byId(id) {
  return ARCHETYPES.find((a) => a.id === id);
}

function validate(archetype, content) {
  return archetype.fields
    .filter((f) => f.required)
    .filter((f) => {
      const value = content?.[f.name];
      if (value === undefined || value === null) return true;
      return typeof value === "string" ? value.trim() === "" : value.length === 0;
    })
    .map((f) => ({ field: f.name, message: `missing required field: ${f.name}` }));
}

function esc(s) {
  return String(s)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
}

/**
 * Mirrors the real renderer's print contract: margins from `@page`, a
 * `.page` with no fixed height, no aspect-ratio and no overflow, and
 * `break-inside: avoid` on rows and list items. Keeping the fixture
 * honest about this matters, because the preview being the output rather
 * than a lookalike is the whole rule this row is built under.
 */
function renderHtml(archetype, content) {
  const blocks = archetype.fields
    .map((field) => {
      const value = content?.[field.name];
      if (value === undefined || value === null || value === "" || value.length === 0) return "";
      if (Array.isArray(value)) {
        const items = value
          .map((item) =>
            typeof item === "string"
              ? `<li>${esc(item)}</li>`
              : `<li>${Object.values(item).map(esc).join(" &middot; ")}</li>`,
          )
          .join("\n");
        return `<h2>${esc(field.label)}</h2>\n<ul class="bullets">\n${items}\n</ul>`;
      }
      return `<h2>${esc(field.label)}</h2>\n<p>${esc(value)}</p>`;
    })
    .filter(Boolean)
    .join("\n");

  return `<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>${esc(archetype.title)}</title>
<style>
body { margin: 0; background: #f4f6fa; font-family: Calibri, 'Segoe UI', Arial, sans-serif; color: #000; font-size: 15px; line-height: 1.4; }
.page { max-width: 900px; margin: 24px auto; background: #fff; padding: 63px; box-shadow: 0 1px 4px rgba(0,0,0,0.12); }
h1 { font-size: 27px; margin: 0 0 4px 0; }
h2 { font-size: 19px; border-bottom: 1px solid #ccc; padding-bottom: 4px; margin: 18px 0 8px 0; }
p { margin: 0 0 10px 0; }
ul.bullets { margin: 0 0 10px 0; padding-left: 22px; }
@page { size: A4; margin: 25.4mm; }
@media print {
  body { background: #fff; }
  .page { max-width: none; margin: 0; padding: 0; box-shadow: none; }
  tr, li { break-inside: avoid; }
}
</style>
</head>
<body>
<div class="page">
<h1>${esc(archetype.title)}</h1>
${blocks}
</div>
</body>
</html>
`;
}

function json(res, status, body) {
  const payload = JSON.stringify(body);
  res.writeHead(status, { "Content-Type": "application/json" });
  res.end(payload);
}

async function readBody(req) {
  const chunks = [];
  for await (const chunk of req) chunks.push(chunk);
  return chunks.length ? JSON.parse(Buffer.concat(chunks).toString("utf8")) : {};
}

const server = createServer(async (req, res) => {
  const url = new URL(req.url ?? "/", `http://localhost:${PORT}`);
  const path = url.pathname;

  if (path === "/health") return json(res, 200, { status: "ok" });

  if (path === "/publications") {
    return json(res, 200, { publications: [] });
  }

  if (path === "/archetypes" && req.method === "GET") {
    return json(res, 200, {
      archetypes: ARCHETYPES.map(({ id, title, governance, layout }) => ({
        id,
        title,
        governance,
        layout,
      })),
    });
  }

  const schemaMatch = /^\/archetypes\/([^/]+)\/schema$/.exec(path);
  if (schemaMatch && req.method === "GET") {
    const archetype = byId(decodeURIComponent(schemaMatch[1]));
    if (!archetype) return json(res, 404, { error: `no archetype "${schemaMatch[1]}"` });
    return json(res, 200, archetype);
  }

  if (path === "/archetypes/preview" && req.method === "POST") {
    const body = await readBody(req);
    const archetype = byId(body.archetypeId);
    if (!archetype) return json(res, 404, { error: `no archetype "${body.archetypeId}"` });
    const errors = validate(archetype, body.content);
    if (errors.length) return json(res, 422, { errors });
    return json(res, 200, { html: renderHtml(archetype, body.content) });
  }

  if (path === "/archetypes/generate" && req.method === "POST") {
    const body = await readBody(req);
    const archetype = byId(body.archetypeId);
    if (!archetype) return json(res, 404, { error: `no archetype "${body.archetypeId}"` });
    const errors = validate(archetype, body.content);
    if (errors.length) return json(res, 422, { errors });
    const html = renderHtml(archetype, body.content);
    const files = {};
    for (const format of body.formats ?? ["html"]) {
      const text = format === "html" ? html : `# ${archetype.title}\n\n(fixture output for .${format})\n`;
      const base64 = Buffer.from(text, "utf8").toString("base64");
      files[format] = {
        filename: `${archetype.id}.${format}`,
        bytes: Buffer.byteLength(text, "utf8"),
        base64,
      };
    }
    return json(res, 200, { archetypeId: archetype.id, files });
  }

  const draftMatch = /^\/documents\/drafts\/(.+)$/.exec(path);
  if (draftMatch && req.method === "GET") {
    const key = decodeURIComponent(draftMatch[1]);
    if (!drafts.has(key)) return json(res, 404, { error: "no draft" });
    return json(res, 200, drafts.get(key));
  }
  if (path === "/documents/drafts" && req.method === "PUT") {
    const draft = await readBody(req);
    if (!draft.key) return json(res, 400, { error: "key is required" });
    drafts.set(draft.key, draft);
    return json(res, 200, draft);
  }

  return json(res, 404, { error: `no route ${req.method} ${path}` });
});

server.listen(PORT, () => {
  process.stdout.write(`mock engine listening on http://localhost:${PORT}\n`);
});
