// Plain `node --test` (Node 22+ strips the TypeScript types on import): no @/ aliases in lib/canvas.ts.
import test from "node:test";
import assert from "node:assert/strict";
import * as canvas from "./canvas.ts";

const SCHEMA = { id: "memo", title: "Memo", governance: false, layout: "header-block", filename_fields: ["re", "date"],
  fields: [{ name: "to", label: "To", type: "text", required: true, section: null, keys: null },
           { name: "opts", label: "Opts", type: "reasoned-list", required: false, section: "S", keys: ["option", "detail"] }] };

function withEnv(env, fn) {
  const saved = { ...process.env };
  Object.assign(process.env, env);
  for (const [k, v] of Object.entries(env)) if (v === undefined) delete process.env[k];
  return Promise.resolve(fn()).finally(() => { for (const k of Object.keys(env)) { if (saved[k] === undefined) delete process.env[k]; else process.env[k] = saved[k]; } });
}

test("a Canvas problem string becomes per-field errors the wizard can bind", () => {
  const e = canvas.toFieldErrors("single-page-memo: missing required field: to; missing required field: body_paragraphs");
  assert.deepEqual(e, [{ field: "to", message: "to is required" }, { field: "body_paragraphs", message: "body paragraphs is required" }]);
  assert.deepEqual(canvas.toFieldErrors("something unrelated"), []);
});

test("the Canvas schema maps to the wizard's ArchetypeSchema", () => {
  const s = canvas.toArchetypeSchema(SCHEMA);
  assert.deepEqual(s.filenameFields, { primary: "re", secondary: "date" });
  assert.equal(s.layout, "header-block");
  assert.deepEqual(s.fields[0], { name: "to", label: "To", type: "text", required: true });
  assert.deepEqual(s.fields[1], { name: "opts", label: "Opts", type: "reasoned-list", required: false, section: "S", keys: ["option", "detail"] });
});

test("without a token nothing is sent and the caller gets a clear 503", async () => {
  await withEnv({ CANVAS_RENDER_TOKEN: undefined }, async () => {
    let called = false;
    const orig = globalThis.fetch; globalThis.fetch = async () => { called = true; };
    try {
      const r = await canvas.canvasFetch("/v1/archetypes");
      assert.equal(r.status, 503);
      assert.match((await r.json()).error, /CANVAS_RENDER_TOKEN is not set/);
      assert.equal(called, false);
    } finally { globalThis.fetch = orig; }
  });
});

test("requests carry the render-only bearer token to the configured Canvas", async () => {
  await withEnv({ CANVAS_RENDER_TOKEN: "tok", CANVAS_URL: "https://c.example/" }, async () => {
    const seen = []; const orig = globalThis.fetch;
    globalThis.fetch = async (u, i) => { seen.push({ u, i }); return Response.json({ ok: true }); };
    try {
      await canvas.canvasFetch("/v1/render", { method: "POST", body: { a: 1 } });
      assert.equal(seen[0].u, "https://c.example/v1/render");
      assert.equal(seen[0].i.headers.Authorization, "Bearer tok");
      assert.equal(seen[0].i.body, '{"a":1}');
    } finally { globalThis.fetch = orig; }
  });
});

test("a network failure is a 502 that names the engine and never leaks the token", async () => {
  await withEnv({ CANVAS_RENDER_TOKEN: "sekrit" }, async () => {
    const orig = globalThis.fetch; globalThis.fetch = async () => { throw new Error("ECONNREFUSED"); };
    try {
      const r = await canvas.canvasFetch("/v1/archetypes");
      assert.equal(r.status, 502);
      const body = JSON.stringify(await r.json());
      assert.match(body, /document engine/); assert.doesNotMatch(body, /sekrit/);
    } finally { globalThis.fetch = orig; }
  });
});

test("listSchemas fetches every schema once and caches them", async () => {
  await withEnv({ CANVAS_RENDER_TOKEN: "t" }, async () => {
    canvas.resetCache(); let n = 0; const orig = globalThis.fetch;
    globalThis.fetch = async (u) => { n++; return new URL(u).pathname === "/v1/archetypes" ? Response.json({ archetypes: [{ id: "memo" }] }) : Response.json(SCHEMA); };
    try {
      const a = await canvas.listSchemas(); const b = await canvas.listSchemas();
      assert.equal(a.length, 1); assert.equal(b, a); assert.equal(n, 2);
    } finally { globalThis.fetch = orig; canvas.resetCache(); }
  });
});

test("a Canvas validation failure is relayed with the status and per-field errors", async () => {
  const upstream = Response.json({ error: "single-page-memo: missing required field: to" }, { status: 422 });
  const r = await canvas.relay(upstream);
  assert.equal(r.status, 422);
  assert.deepEqual(await r.json(), { error: "single-page-memo: missing required field: to", errors: [{ field: "to", message: "to is required" }] });
});
