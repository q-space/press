"use client";

import type { ReactNode } from "react";

import { FieldDef, placeholderFor } from "@/lib/archetypes";

const BASE =
  "w-full rounded border bg-white px-2.5 py-1.5 text-sm text-neutral-900 outline-none " +
  "focus:border-neutral-900 focus:ring-1 focus:ring-neutral-900";

/**
 * One control per field, chosen from the field's declared type -- the
 * whole form is generated from the archetype's own field definitions, so
 * a new archetype needs no UI work at all.
 *
 * A validation error binds to its own control here rather than surfacing
 * as one opaque form-level failure, which is the point of the engine
 * returning an error array in the first place.
 */
export default function FieldControl({
  field,
  value,
  error,
  onChange,
  compact = false,
}: {
  field: FieldDef;
  value: string;
  error?: string;
  onChange: (next: string) => void;
  compact?: boolean;
}) {
  const id = `field-${field.name}`;
  const borderClass = error ? "border-red-500" : "border-neutral-300";
  const describedBy = error ? `${id}-error` : undefined;

  let control: ReactNode;
  if (field.type === "select") {
    control = (
      <select
        id={id}
        className={`${BASE} ${borderClass}`}
        value={value}
        aria-invalid={Boolean(error)}
        aria-describedby={describedBy}
        onChange={(e) => onChange(e.target.value)}
      >
        <option value="">Select…</option>
        {(field.options ?? []).map((option) => (
          <option key={option} value={option}>
            {option}
          </option>
        ))}
      </select>
    );
  } else if (field.type === "text") {
    control = (
      <input
        id={id}
        type="text"
        className={`${BASE} ${borderClass}`}
        value={value}
        aria-invalid={Boolean(error)}
        aria-describedby={describedBy}
        onChange={(e) => onChange(e.target.value)}
      />
    );
  } else {
    // textarea, list, reasoned-list and table-list are all line-oriented
    // text entry; what differs is how lib/archetypes.ts parses the lines.
    const isStructured = field.type !== "textarea";
    control = (
      <textarea
        id={id}
        rows={compact ? 2 : isStructured ? 4 : 3}
        placeholder={placeholderFor(field)}
        className={`${BASE} ${borderClass} ${isStructured ? "font-mono text-xs" : ""}`}
        value={value}
        aria-invalid={Boolean(error)}
        aria-describedby={describedBy}
        onChange={(e) => onChange(e.target.value)}
      />
    );
  }

  return (
    <div className={compact ? "flex items-center gap-3" : "flex flex-col gap-1"}>
      <label
        htmlFor={id}
        className={`text-xs font-medium text-neutral-600 ${compact ? "w-24 shrink-0 text-right" : ""}`}
      >
        {field.label || field.name}
        {field.required ? <span className="ml-0.5 text-red-600">*</span> : null}
      </label>
      <div className="min-w-0 flex-1">
        {control}
        {error ? (
          <p id={`${id}-error`} className="mt-1 text-xs text-red-600">
            {error}
          </p>
        ) : null}
      </div>
    </div>
  );
}
