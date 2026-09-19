"use client";

import { useRef } from "react";

/**
 * The live preview is the engine's own HTML output, rendered as-is in an
 * isolated frame -- not a second renderer that approximates it.
 *
 * That isolation is the point. The engine returns a self-contained
 * document (inline `<style>`, including its `@page` rule and print
 * media query), so anything that reached into it -- Tailwind's preflight,
 * the studio's own layout -- would make the preview and the exported PDF
 * two different documents. Printing this frame and exporting a PDF run
 * the same stylesheet over the same markup.
 *
 * The frame is sandboxed without `allow-scripts`, so nothing in a
 * rendered document executes; `<details>` bullets still expand, since
 * that is native behaviour rather than script.
 */
export default function PreviewPane({ html, empty }: { html: string; empty: string }) {
  const frame = useRef<HTMLIFrameElement>(null);

  return (
    <div className="flex min-h-0 flex-1 flex-col gap-2">
      <div className="flex items-center gap-2">
        <span className="text-xs font-semibold uppercase tracking-wide text-neutral-500">Preview</span>
        <button
          type="button"
          disabled={!html}
          onClick={() => frame.current?.contentWindow?.print()}
          className="ml-auto rounded border border-neutral-300 px-2.5 py-1 text-xs text-neutral-700 hover:border-neutral-900 disabled:opacity-40"
        >
          Print / Save as PDF
        </button>
      </div>
      {html ? (
        <iframe
          ref={frame}
          title="Document preview"
          srcDoc={html}
          sandbox="allow-same-origin allow-modals"
          className="min-h-[520px] w-full flex-1 rounded border border-neutral-200 bg-white"
        />
      ) : (
        <div className="flex min-h-[520px] flex-1 items-center justify-center rounded border border-dashed border-neutral-300 p-6 text-center text-sm text-neutral-500">
          {empty}
        </div>
      )}
    </div>
  );
}
