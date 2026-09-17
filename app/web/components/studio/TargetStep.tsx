"use client";

import { Publication } from "@qspace-press/shared";
import { GENERAL_TARGET, StudioTarget } from "@/lib/studio";

/**
 * Step 1 -- who the document is for, asked before which shape it takes.
 *
 * The answer does two things: it resolves the archetype namespace, so a
 * publication's own document shapes appear alongside the general set; and
 * it gives the studio something to pre-fill the archetype's filename
 * slots from, rather than making the creator retype what the wizard
 * already knows.
 */
export default function TargetStep({
  publications,
  loading,
  error,
  onPick,
}: {
  publications: Publication[];
  loading: boolean;
  error: string;
  onPick: (target: StudioTarget) => void;
}) {
  return (
    <div className="flex flex-col gap-4">
      <div>
        <h2 className="text-lg font-semibold text-neutral-900">Draft for</h2>
        <p className="mt-1 text-sm text-neutral-600">
          Pick who this document is for. It decides which archetypes are available.
        </p>
      </div>

      <div className="grid gap-3 sm:grid-cols-2">
        <button
          type="button"
          onClick={() => onPick(GENERAL_TARGET)}
          className="rounded border border-neutral-300 p-4 text-left hover:border-neutral-900"
        >
          <div className="text-sm font-semibold text-neutral-900">General</div>
          <div className="mt-1 text-xs text-neutral-600">
            No target. Drafts go straight to the general (<code>_common</code>) archetype set.
          </div>
        </button>

        {publications.map((publication) => (
          <button
            key={publication.id}
            type="button"
            onClick={() =>
              onPick({ kind: "publication", id: publication.id, label: publication.displayName })
            }
            className="rounded border border-neutral-300 p-4 text-left hover:border-neutral-900"
          >
            <div className="text-sm font-semibold text-neutral-900">{publication.displayName}</div>
            <div className="mt-1 font-mono text-xs text-neutral-600">@{publication.handle}</div>
          </button>
        ))}
      </div>

      {loading ? <p className="text-xs text-neutral-500">Loading your publications…</p> : null}
      {error ? (
        <p className="text-xs text-amber-700">
          Publications could not be loaded ({error}). You can still draft against the general set.
        </p>
      ) : null}
    </div>
  );
}
