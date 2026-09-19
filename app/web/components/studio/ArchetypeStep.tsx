"use client";

import { ArchetypeSummary } from "@/lib/archetypes";

/**
 * Step 2 -- which document shape.
 *
 * iSconl's picker sorts by recency from a documents list it never
 * actually populated, so the ordering was always a no-op. Until Qpress
 * has real document history to sort by (`BB26091205` onwards), this shows
 * the registry's order as it comes, which is at least honest about what
 * it knows.
 */
export default function ArchetypeStep({
  archetypes,
  namespace,
  loading,
  error,
  onPick,
  onBack,
}: {
  archetypes: ArchetypeSummary[];
  namespace: string;
  loading: boolean;
  error: string;
  onPick: (id: string) => void;
  onBack: () => void;
}) {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start gap-3">
        <div className="flex-1">
          <h2 className="text-lg font-semibold text-neutral-900">Choose an archetype</h2>
          <p className="mt-1 font-mono text-xs text-neutral-500">namespace: {namespace}</p>
        </div>
        <button
          type="button"
          onClick={onBack}
          className="rounded border border-neutral-300 px-2.5 py-1 text-xs text-neutral-700 hover:border-neutral-900"
        >
          ← Back
        </button>
      </div>

      {loading ? <p className="text-sm text-neutral-500">Reading the archetype registry…</p> : null}
      {error ? <p className="text-sm text-red-600">{error}</p> : null}

      {!loading && !error && archetypes.length === 0 ? (
        <p className="text-sm text-neutral-600">
          No archetypes in <code>{namespace}</code> (or <code>_common</code>).
        </p>
      ) : null}

      <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
        {archetypes.map((archetype) => (
          <button
            key={archetype.id}
            type="button"
            onClick={() => onPick(archetype.id)}
            className="rounded border border-neutral-300 p-4 text-left hover:border-neutral-900"
          >
            <div className="text-sm font-semibold text-neutral-900">{archetype.title}</div>
            <div className="mt-1 font-mono text-xs text-neutral-500">{archetype.id}</div>
            {archetype.governance ? (
              <div className="mt-2 inline-block rounded bg-neutral-100 px-1.5 py-0.5 text-[10px] uppercase tracking-wide text-neutral-600">
                governance
              </div>
            ) : null}
          </button>
        ))}
      </div>
    </div>
  );
}
