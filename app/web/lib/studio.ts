/** Wizard-level types shared by the three steps. */

export interface StudioTarget {
  kind: "general" | "publication";
  /** Publication id, or "" for the general target. */
  id: string;
  label: string;
}

export const GENERAL_TARGET: StudioTarget = {
  kind: "general",
  id: "",
  label: "General",
};

/**
 * Which archetype set a target resolves to. A publication's own id is its
 * namespace, so an archetype belonging to one publication never shows up
 * while drafting for another; everything falls back to `_common`, which
 * is the only populated namespace today.
 */
export function namespaceFor(target: StudioTarget): string {
  return target.kind === "publication" && target.id ? target.id : "_common";
}

/** Identifies one autosaved draft: this target, this archetype. */
export function draftKey(target: StudioTarget, archetypeId: string): string {
  return `${target.kind}:${target.id || "_"}:${archetypeId}`;
}

export interface StudioDraft {
  key: string;
  archetypeId: string;
  namespace: string;
  values: Record<string, string>;
  savedAt: string;
}
