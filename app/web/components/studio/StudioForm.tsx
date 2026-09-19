"use client";

import { ArchetypeSchema, FieldDef } from "@/lib/archetypes";
import FieldControl from "./FieldControl";

/**
 * One generic form renderer that adapts to hints on the archetype rather
 * than a bespoke form per archetype.
 *
 * - `layout: "header-block"` renders every field as a compact single row
 *   (the to/cc/subject shape an email wants).
 * - otherwise, fields carrying a `section` group into collapsible
 *   fieldsets, and unsectioned fields render flat in their original
 *   position relative to those groups.
 * - an archetype with neither hint renders flat and stacked, which is
 *   every archetype in `_common` today.
 */
export default function StudioForm({
  schema,
  values,
  errors,
  onChange,
}: {
  schema: ArchetypeSchema;
  values: Record<string, string>;
  errors: Record<string, string>;
  onChange: (name: string, next: string) => void;
}) {
  const fields = schema.fields.filter((f) => f.type !== "section");

  const render = (field: FieldDef, compact = false) => (
    <FieldControl
      key={field.name}
      field={field}
      compact={compact}
      value={values[field.name] ?? ""}
      error={errors[field.name]}
      onChange={(next) => onChange(field.name, next)}
    />
  );

  if (schema.layout === "header-block") {
    return <div className="flex flex-col gap-2">{fields.map((f) => render(f, true))}</div>;
  }

  if (!fields.some((f) => f.section)) {
    return <div className="flex flex-col gap-3">{fields.map((f) => render(f))}</div>;
  }

  // Preserve the archetype's own field order: walk once and start a new
  // group whenever the section name changes, rather than bucketing by
  // section name (which would reorder fields that revisit a section).
  const groups: Array<{ section: string | null; fields: FieldDef[] }> = [];
  for (const field of fields) {
    const section = field.section ?? null;
    const last = groups[groups.length - 1];
    if (last && last.section === section) last.fields.push(field);
    else groups.push({ section, fields: [field] });
  }

  return (
    <div className="flex flex-col gap-3">
      {groups.map((group, index) =>
        group.section === null ? (
          <div key={`flat-${index}`} className="flex flex-col gap-3">
            {group.fields.map((f) => render(f))}
          </div>
        ) : (
          <details key={`${group.section}-${index}`} open className="rounded border border-neutral-200">
            <summary className="cursor-pointer select-none px-3 py-2 text-xs font-semibold uppercase tracking-wide text-neutral-600">
              {group.section}
            </summary>
            <div className="flex flex-col gap-3 border-t border-neutral-200 p-3">
              {group.fields.map((f) => render(f))}
            </div>
          </details>
        ),
      )}
    </div>
  );
}
