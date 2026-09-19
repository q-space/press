"use client";

import { useCallback, useEffect, useRef, useState } from "react";
import { CanvasArtifact, publicCanvasUrl, updateCanvas } from "@/lib/canvas";

const AUTOSAVE_DELAY_MS = 800;

/**
 * Canvas's live-editable surface (BB26091502). The published artifact is
 * not a frozen render -- this is the same self-contained HTML document
 * the engine produced (inline `@page`/print styles per BB26091204), made
 * directly editable in place and autosaved back to the account, the same
 * "the account, not the browser" discipline the wizard's own draft
 * autosave already uses.
 *
 * Editing happens inside the SAME sandboxed iframe the studio's preview
 * uses (`allow-same-origin`, no `allow-scripts`) rather than lifting the
 * markup into a rich-text editor's own model -- Canvas takes arbitrary
 * rendered HTML from any caller and must not need to understand its
 * shape (tables, page-break rules, whatever an archetype or another
 * product's renderer produced) to make it editable.
 */
export default function CanvasEditor({ canvas }: { canvas: CanvasArtifact }) {
  const frame = useRef<HTMLIFrameElement>(null);
  const saveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [title, setTitle] = useState(canvas.title);
  const [savedAt, setSavedAt] = useState("");
  const [saveError, setSaveError] = useState("");
  const [copied, setCopied] = useState(false);

  const scheduleSave = useCallback(
    (patch: { title?: string; html?: string }) => {
      if (saveTimer.current) clearTimeout(saveTimer.current);
      saveTimer.current = setTimeout(async () => {
        try {
          const updated = await updateCanvas(canvas.id, patch);
          setSavedAt(updated.updated_at);
          setSaveError("");
        } catch (cause) {
          setSaveError(cause instanceof Error ? cause.message : String(cause));
        }
      }, AUTOSAVE_DELAY_MS);
    },
    [canvas.id],
  );

  useEffect(() => () => {
    if (saveTimer.current) clearTimeout(saveTimer.current);
  }, []);

  function onFrameLoad() {
    const doc = frame.current?.contentDocument;
    if (!doc) return;
    doc.designMode = "on";
    doc.body.addEventListener("input", () => {
      scheduleSave({ html: doc.documentElement.outerHTML });
    });
  }

  function onTitleChange(next: string) {
    setTitle(next);
    scheduleSave({ title: next });
  }

  async function copyShareLink() {
    const url = `${window.location.origin}${publicCanvasUrl(canvas.id)}`;
    try {
      await navigator.clipboard.writeText(url);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      // Clipboard permission denied or unavailable -- the link is still
      // visible in the field below, so this is a convenience loss only.
    }
  }

  return (
    <div className="mx-auto flex w-full max-w-5xl flex-col gap-4 p-6">
      <div className="flex flex-wrap items-center gap-3">
        <input
          value={title}
          onChange={(e) => onTitleChange(e.target.value)}
          className="flex-1 rounded border border-neutral-300 px-2.5 py-1 text-lg font-semibold text-neutral-900"
          placeholder="Untitled canvas"
        />
        <span className="text-xs text-neutral-500">
          {saveError ? (
            <span className="text-red-600">Not saved: {saveError}</span>
          ) : savedAt ? (
            `Saved ${new Date(savedAt).toLocaleTimeString()}`
          ) : (
            "Editable -- changes save automatically"
          )}
        </span>
      </div>

      <div className="flex flex-wrap items-center gap-2 rounded border border-neutral-200 bg-neutral-50 p-2">
        <span className="text-xs font-semibold uppercase tracking-wide text-neutral-500">
          Share
        </span>
        <code className="flex-1 truncate rounded bg-white px-2 py-1 text-xs text-neutral-700">
          {publicCanvasUrl(canvas.id)}
        </code>
        <button
          type="button"
          onClick={copyShareLink}
          className="rounded border border-neutral-300 px-2.5 py-1 text-xs text-neutral-700 hover:border-neutral-900"
        >
          {copied ? "Copied" : "Copy link"}
        </button>
      </div>

      <iframe
        ref={frame}
        title="Canvas editor"
        srcDoc={canvas.content_html}
        sandbox="allow-same-origin"
        onLoad={onFrameLoad}
        className="min-h-[640px] w-full flex-1 rounded border border-neutral-200 bg-white"
      />
    </div>
  );
}
