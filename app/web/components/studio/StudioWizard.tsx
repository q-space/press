"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { Publication } from "@qspace-press/shared";
import {
  ArchetypeSchema,
  ArchetypeSummary,
  FieldError,
  GenerateResponse,
  GeneratedFile,
  OUTPUT_FORMATS,
  OutputFormat,
  PreviewResponse,
  buildContentPayload,
  errorsByField,
  normalizeErrors,
  validateRequired,
} from "@/lib/archetypes";
import {
  DEFAULT_ACCENT,
  GENERAL_TARGET,
  StudioDraft,
  StudioTarget,
  draftKey,
  namespaceFor,
} from "@/lib/studio";
import ArchetypeStep from "./ArchetypeStep";
import PreviewPane from "./PreviewPane";
import StudioForm from "./StudioForm";
import TargetStep from "./TargetStep";

const MIME: Record<OutputFormat, string> = {
  md: "text/markdown",
  html: "text/html",
  pdf: "application/pdf",
  docx: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
};

const AUTOSAVE_DELAY_MS = 800;

async function readError(res: Response): Promise<string> {
  try {
    const body = (await res.json()) as { error?: string };
    if (body?.error) return body.error;
  } catch {
    // fall through to the status line
  }
  return `${res.status} ${res.statusText}`;
}

function downloadFile(format: OutputFormat, file: GeneratedFile) {
  const binary = atob(file.base64);
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  const url = URL.createObjectURL(new Blob([bytes], { type: MIME[format] }));
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = file.filename;
  anchor.click();
  URL.revokeObjectURL(url);
}

/**
 * Creator Studio's archetype wizard (BB26091204): target -> archetype ->
 * studio, with a generated form on the left and the engine's own rendered
 * output on the right.
 *
 * This is the structured-document surface, not a rich-text editor. Qpress
 * has both: Tiptap for freeform posts, this for documents whose shape is
 * declared by an archetype.
 */
export default function StudioWizard() {
  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [target, setTarget] = useState<StudioTarget>(GENERAL_TARGET);

  const [publications, setPublications] = useState<Publication[]>([]);
  const [publicationsLoading, setPublicationsLoading] = useState(true);
  const [publicationsError, setPublicationsError] = useState("");

  const [archetypes, setArchetypes] = useState<ArchetypeSummary[]>([]);
  const [archetypesLoading, setArchetypesLoading] = useState(false);
  const [archetypesError, setArchetypesError] = useState("");

  const [schema, setSchema] = useState<ArchetypeSchema | null>(null);
  const [values, setValues] = useState<Record<string, string>>({});
  const [fieldErrors, setFieldErrors] = useState<Record<string, string>>({});
  const [formErrors, setFormErrors] = useState<string[]>([]);

  const [previewHtml, setPreviewHtml] = useState("");
  const [previewBusy, setPreviewBusy] = useState(false);
  const [formats, setFormats] = useState<Record<OutputFormat, boolean>>({
    md: false,
    docx: true,
    pdf: true,
    html: false,
  });
  const [result, setResult] = useState<GenerateResponse | null>(null);
  const [generateBusy, setGenerateBusy] = useState(false);
  const [notice, setNotice] = useState("");
  const [draftSavedAt, setDraftSavedAt] = useState("");

  // BB26091501 -- accent colour. `accentOverrideOn` off means "use the
  // brand default (or the house default, for General)"; the resolution
  // order (per-document override, else brand default) is applied
  // server-side against exactly these two inputs, so leaving this off
  // must send no override at all, not the pre-filled value re-sent as
  // one -- otherwise every document would silently "override" to its own
  // brand default, which is indistinguishable from not overriding until
  // the brand default itself changes.
  const [accentOverrideOn, setAccentOverrideOn] = useState(false);
  const [accentOverride, setAccentOverride] = useState(DEFAULT_ACCENT);
  const brandDefaultAccent = target.accentColor || DEFAULT_ACCENT;

  const namespace = namespaceFor(target);
  const autosaveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    let cancelled = false;
    (async () => {
      try {
        const res = await fetch("/api/publications", { cache: "no-store" });
        if (!res.ok) throw new Error(await readError(res));
        const body = (await res.json()) as { publications?: Publication[] };
        if (!cancelled) setPublications(body.publications ?? []);
      } catch (cause) {
        if (!cancelled) setPublicationsError(cause instanceof Error ? cause.message : String(cause));
      } finally {
        if (!cancelled) setPublicationsLoading(false);
      }
    })();
    return () => {
      cancelled = true;
    };
  }, []);

  const loadArchetypes = useCallback(async (ns: string) => {
    setArchetypesLoading(true);
    setArchetypesError("");
    try {
      const res = await fetch(`/api/archetypes?namespace=${encodeURIComponent(ns)}`, {
        cache: "no-store",
      });
      if (!res.ok) throw new Error(await readError(res));
      const body = (await res.json()) as { archetypes?: ArchetypeSummary[] };
      setArchetypes(body.archetypes ?? []);
    } catch (cause) {
      setArchetypes([]);
      setArchetypesError(cause instanceof Error ? cause.message : String(cause));
    } finally {
      setArchetypesLoading(false);
    }
  }, []);

  /**
   * Autosave to the account, not the browser: a draft started on a laptop
   * is still there on a phone. Debounced, so a keystroke is not a write.
   */
  const scheduleAutosave = useCallback(
    (nextValues: Record<string, string>, activeSchema: ArchetypeSchema | null) => {
      if (!activeSchema) return;
      if (autosaveTimer.current) clearTimeout(autosaveTimer.current);
      autosaveTimer.current = setTimeout(async () => {
        const draft: StudioDraft = {
          key: draftKey(target, activeSchema.id),
          archetypeId: activeSchema.id,
          namespace,
          values: nextValues,
          savedAt: new Date().toISOString(),
        };
        try {
          const res = await fetch("/api/drafts", {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(draft),
          });
          setDraftSavedAt(res.ok ? draft.savedAt : "");
        } catch {
          setDraftSavedAt("");
        }
      }, AUTOSAVE_DELAY_MS);
    },
    [namespace, target],
  );

  useEffect(
    () => () => {
      if (autosaveTimer.current) clearTimeout(autosaveTimer.current);
    },
    [],
  );

  async function pickTarget(next: StudioTarget) {
    setTarget(next);
    setStep(2);
    await loadArchetypes(namespaceFor(next));
  }

  async function pickArchetype(id: string) {
    setArchetypesError("");
    try {
      const res = await fetch(`/api/archetypes/${encodeURIComponent(id)}/schema`, {
        cache: "no-store",
      });
      if (!res.ok) throw new Error(await readError(res));
      const nextSchema = (await res.json()) as ArchetypeSchema;

      // Pre-fill the archetype's own filename slots from what the wizard
      // already knows, rather than asking the creator to retype it.
      const seeded: Record<string, string> = {};
      const today = new Date().toISOString().slice(0, 10);
      for (const field of nextSchema.fields) {
        if (field.name === nextSchema.filenameFields?.primary && target.kind === "publication") {
          seeded[field.name] = target.label;
        }
        if (field.type === "text" && /^dated?$/i.test(field.name)) seeded[field.name] = today;
      }

      let restored: Record<string, string> | null = null;
      try {
        const draftRes = await fetch(`/api/drafts?key=${encodeURIComponent(draftKey(target, id))}`, {
          cache: "no-store",
        });
        if (draftRes.ok) {
          const draft = (await draftRes.json()) as StudioDraft | null;
          if (draft?.values) restored = draft.values;
        }
      } catch {
        // No saved draft, or drafts are unreachable. Start clean rather
        // than blocking the creator on a convenience feature.
      }

      setSchema(nextSchema);
      setValues(restored ?? seeded);
      setDraftSavedAt(restored ? "restored" : "");
      setFieldErrors({});
      setFormErrors([]);
      setPreviewHtml("");
      setResult(null);
      setNotice(restored ? "An autosaved draft for this document was restored." : "");
      // A fresh document starts on the brand default, not carrying over
      // whatever override the previous document happened to have.
      setAccentOverrideOn(false);
      setAccentOverride(target.accentColor || DEFAULT_ACCENT);
      setStep(3);
    } catch (cause) {
      setArchetypesError(cause instanceof Error ? cause.message : String(cause));
    }
  }

  function changeField(name: string, next: string) {
    setValues((current) => {
      const updated = { ...current, [name]: next };
      scheduleAutosave(updated, schema);
      return updated;
    });
    setFieldErrors((current) => {
      if (!current[name]) return current;
      const rest = { ...current };
      delete rest[name];
      return rest;
    });
  }

  function applyErrors(errors: FieldError[]): boolean {
    setFieldErrors(errorsByField(errors));
    setFormErrors(errors.filter((e) => !e.field).map((e) => e.message));
    return errors.length === 0;
  }

  function checkBeforeSend(): boolean {
    if (!schema) return false;
    return applyErrors(validateRequired(schema, values));
  }

  async function runPreview() {
    if (!schema || !checkBeforeSend()) return;
    setPreviewBusy(true);
    setNotice("");
    try {
      const res = await fetch("/api/archetypes/preview", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          namespace,
          archetypeId: schema.id,
          content: buildContentPayload(schema, values),
          // Only present when the creator actually turned the override
          // on for this document -- an absent field is what lets the
          // engine's resolution order fall through to the brand default.
          accentOverride: accentOverrideOn ? accentOverride : undefined,
        }),
      });
      const body = (await res.json()) as PreviewResponse & { error?: string };
      if (!res.ok) {
        const errors = normalizeErrors(body.errors, schema);
        if (errors.length) applyErrors(errors);
        else setFormErrors([body.error ?? `${res.status} ${res.statusText}`]);
        return;
      }
      setPreviewHtml(body.html ?? "");
    } catch (cause) {
      setFormErrors([cause instanceof Error ? cause.message : String(cause)]);
    } finally {
      setPreviewBusy(false);
    }
  }

  async function runGenerate() {
    if (!schema || !checkBeforeSend()) return;
    const selected = OUTPUT_FORMATS.filter((f) => formats[f]);
    if (selected.length === 0) {
      setFormErrors(["Pick at least one output format."]);
      return;
    }
    setGenerateBusy(true);
    setNotice("");
    try {
      const res = await fetch("/api/archetypes/generate", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          namespace,
          archetypeId: schema.id,
          formats: selected,
          content: buildContentPayload(schema, values),
          accentOverride: accentOverrideOn ? accentOverride : undefined,
        }),
      });
      const body = (await res.json()) as GenerateResponse & { error?: string };
      if (!res.ok) {
        const errors = normalizeErrors(body.errors, schema);
        if (errors.length) applyErrors(errors);
        else setFormErrors([body.error ?? `${res.status} ${res.statusText}`]);
        return;
      }
      setResult(body);
    } catch (cause) {
      setFormErrors([cause instanceof Error ? cause.message : String(cause)]);
    } finally {
      setGenerateBusy(false);
    }
  }

  const crumbs: Array<{ n: 1 | 2 | 3; label: string }> = [
    { n: 1, label: target.kind === "publication" ? target.label : "General" },
    { n: 2, label: schema ? schema.title : "Archetype" },
    { n: 3, label: "Studio" },
  ];

  return (
    <div className="mx-auto flex w-full max-w-7xl flex-col gap-6 p-6">
      <nav className="flex items-center gap-2 text-xs text-neutral-500">
        {crumbs.map((crumb, index) => (
          <span key={crumb.n} className="flex items-center gap-2">
            {index > 0 ? <span aria-hidden>/</span> : null}
            <button
              type="button"
              disabled={crumb.n > step}
              onClick={() => setStep(crumb.n)}
              className={`rounded px-1.5 py-0.5 disabled:cursor-default disabled:opacity-40 ${
                crumb.n === step ? "font-semibold text-neutral-900" : "hover:text-neutral-900"
              }`}
            >
              {crumb.label}
            </button>
          </span>
        ))}
      </nav>

      {step === 1 ? (
        <TargetStep
          publications={publications}
          loading={publicationsLoading}
          error={publicationsError}
          onPick={pickTarget}
        />
      ) : null}

      {step === 2 ? (
        <ArchetypeStep
          archetypes={archetypes}
          namespace={namespace}
          loading={archetypesLoading}
          error={archetypesError}
          onPick={pickArchetype}
          onBack={() => setStep(1)}
        />
      ) : null}

      {step === 3 && schema ? (
        <div className="flex flex-col gap-4">
          <div className="flex flex-wrap items-start gap-3">
            <div className="flex-1">
              <h2 className="text-lg font-semibold text-neutral-900">{schema.title}</h2>
              <p className="font-mono text-xs text-neutral-500">
                {namespace} / {schema.id}
              </p>
              {target.kind === "publication" ? (
                <p className="mt-0.5 text-xs text-neutral-600">
                  Drafting for <strong>{target.label}</strong>
                </p>
              ) : null}
            </div>
            <span className="text-xs text-neutral-500">
              {draftSavedAt === "restored"
                ? "Draft restored"
                : draftSavedAt
                  ? `Draft saved ${new Date(draftSavedAt).toLocaleTimeString()}`
                  : ""}
            </span>
            <button
              type="button"
              onClick={() => setStep(2)}
              className="rounded border border-neutral-300 px-2.5 py-1 text-xs text-neutral-700 hover:border-neutral-900"
            >
              Back to archetypes
            </button>
          </div>

          {notice ? <p className="text-xs text-neutral-600">{notice}</p> : null}
          {formErrors.length ? (
            <ul className="rounded border border-red-300 bg-red-50 p-3 text-xs text-red-700">
              {formErrors.map((message) => (
                <li key={message}>{message}</li>
              ))}
            </ul>
          ) : null}

          <div className="grid min-h-0 gap-5 lg:grid-cols-2">
            <div className="flex max-h-[640px] flex-col gap-3 overflow-y-auto pr-1">
              <StudioForm
                schema={schema}
                values={values}
                errors={fieldErrors}
                onChange={changeField}
              />
            </div>

            <div className="flex min-h-0 flex-col gap-3">
              <div className="flex flex-wrap items-center gap-2 text-xs text-neutral-700">
                <span
                  aria-hidden
                  className="h-3.5 w-3.5 rounded-full border border-neutral-300"
                  style={{ backgroundColor: `#${accentOverrideOn ? accentOverride : brandDefaultAccent}` }}
                />
                <span>
                  Accent:{" "}
                  {target.kind === "publication" ? `${target.label}'s default` : "house default"}
                  {" "}(#{brandDefaultAccent})
                </span>
                <label className="ml-2 flex items-center gap-1">
                  <input
                    type="checkbox"
                    checked={accentOverrideOn}
                    onChange={(e) => {
                      setAccentOverrideOn(e.target.checked);
                      // Seed the override input from the brand default the
                      // first time it's turned on, rather than whatever it
                      // last held -- editing from there is a real override,
                      // not a blind re-type.
                      if (e.target.checked) setAccentOverride(brandDefaultAccent);
                    }}
                  />
                  Override for this document
                </label>
                {accentOverrideOn ? (
                  <input
                    type="color"
                    value={`#${accentOverride}`}
                    onChange={(e) => setAccentOverride(e.target.value.replace("#", ""))}
                    className="h-6 w-8 rounded border border-neutral-300 p-0"
                    aria-label="Accent colour override for this document"
                  />
                ) : null}
              </div>

              <div className="flex flex-wrap items-center gap-2">
                <button
                  type="button"
                  onClick={runPreview}
                  disabled={previewBusy}
                  className="rounded border border-neutral-300 px-2.5 py-1 text-xs text-neutral-700 hover:border-neutral-900 disabled:opacity-40"
                >
                  {previewBusy ? "Rendering..." : "Live preview"}
                </button>
                <span className="ml-1 text-xs text-neutral-500">Formats:</span>
                {OUTPUT_FORMATS.map((format) => (
                  <label key={format} className="flex items-center gap-1 text-xs text-neutral-700">
                    <input
                      type="checkbox"
                      checked={formats[format]}
                      onChange={(e) =>
                        setFormats((current) => ({ ...current, [format]: e.target.checked }))
                      }
                    />
                    {format}
                  </label>
                ))}
                <button
                  type="button"
                  onClick={runGenerate}
                  disabled={generateBusy}
                  className="ml-auto rounded bg-neutral-900 px-3 py-1 text-xs font-medium text-white hover:bg-neutral-700 disabled:opacity-40"
                >
                  {generateBusy ? "Generating..." : "Generate"}
                </button>
              </div>

              <PreviewPane
                html={previewHtml}
                empty="Click Live preview to see the document exactly as it will export."
              />

              {result ? (
                <div className="flex flex-col gap-2 rounded border border-neutral-200 p-3">
                  <div className="text-xs font-semibold text-neutral-900">Generated files</div>
                  {Object.entries(result.files).map(([format, file]) => (
                    <div key={format} className="flex items-center justify-between gap-3">
                      <span className="truncate font-mono text-xs text-neutral-600">
                        {file.filename} - {(file.bytes / 1024).toFixed(1)} KB
                      </span>
                      <button
                        type="button"
                        onClick={() => downloadFile(format as OutputFormat, file)}
                        className="rounded border border-neutral-300 px-2 py-0.5 text-xs text-neutral-700 hover:border-neutral-900"
                      >
                        Download
                      </button>
                    </div>
                  ))}
                </div>
              ) : null}
            </div>
          </div>
        </div>
      ) : null}
    </div>
  );
}
