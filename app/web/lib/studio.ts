/** Wizard-level types shared by the three steps. */

export interface StudioTarget {
  kind: "general" | "publication";
  /** Publication id, or "" for the general target. */
  id: string;
  label: string;
  /**
   * BB26091501 -- the publication's own default accent (bare hex, no
   * leading `#`), carried along from step 1 so step 3 can pre-fill from
   * it without a second fetch. `null`/absent for the general target,
   * which has no brand and falls back to the house default.
   */
  accentColor?: string | null;
}

export const GENERAL_TARGET: StudioTarget = {
  kind: "general",
  id: "",
  label: "General",
  accentColor: null,
};

/**
 * The engine's own house default (`generate::style::DEFAULT_ACCENT` on
 * the Rust side) -- kept in sync by hand since the two sides don't share
 * a crate. Used only to pre-fill the accent input when neither a brand
 * default nor a prior override exists; the actual resolution at
 * generation time happens server-side, in the same order.
 */
export const DEFAULT_ACCENT = "2F5496";

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
