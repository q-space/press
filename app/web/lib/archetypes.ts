/**
 * The archetype contract -- the single place the Creator Studio wizard
 * (BB26091204) knows anything about document shapes.
 *
 * There is deliberately NO local copy of any archetype's field
 * definitions here. Everything comes from the engine over the wire
 * (Canvas, canvas.acexoft.com, via the server-side routes under app/api/archetypes), because a
 * second copy of a field list is a second thing to keep in step, and the
 * engine's copy is the one the renderers actually build from.
 */

export type FieldType =
  | "text"
  | "textarea"
  | "select"
  | "list"
  | "reasoned-list"
  | "table-list"
  | "section";

export interface FieldDef {
  name: string;
  label: string;
  type: FieldType;
  required: boolean;
  /** Groups this field into a named fieldset in the wizard. */
  section?: string | null;
  /** Sub-keys each row of a `reasoned-list`/`table-list` carries. */
  keys?: string[] | null;
  /** Choices for `select`. */
  options?: string[] | null;
}

export interface ArchetypeSummary {
  id: string;
  title: string;
  governance?: boolean;
  /** Renderer hint, e.g. "header-block" for a to/cc/subject shape. */
  layout?: string | null;
}

export interface ArchetypeSchema extends ArchetypeSummary {
  fields: FieldDef[];
  filenameFields: { primary: string; secondary: string };
}

/** What one field contributes to a document's content payload. */
export type ContentValue = string | string[] | Array<Record<string, string>>;
export type Content = Record<string, ContentValue>;

export interface FieldError {
  field: string;
  message: string;
}

export interface PreviewResponse {
  html: string;
  markdown?: string;
  errors?: FieldError[] | string[];
}

export interface GeneratedFile {
  filename: string;
  bytes: number;
  /** base64 of the file body -- downloaded client-side, never written to disk by the browser. */
  base64: string;
}

export interface GenerateResponse {
  archetypeId: string;
  files: Record<string, GeneratedFile>;
  errors?: FieldError[] | string[];
}

export type OutputFormat = "md" | "docx" | "pdf" | "html";
export const OUTPUT_FORMATS: OutputFormat[] = ["md", "docx", "pdf", "html"];

/** Raw textarea/input text -> the value shape `archetype.build()` expects.
 *  One place, so a new field type is taught to the wizard exactly once. */
export function parseFieldValue(field: FieldDef, raw: string | undefined): ContentValue {
  const text = raw ?? "";
  if (field.type === "list") {
    return text
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean);
  }
  if (field.type === "reasoned-list" || field.type === "table-list") {
    const keys = field.keys && field.keys.length ? field.keys : ["a", "b"];
    return text
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean)
      .map((line) => {
        const parts = line.split("|").map((s) => s.trim());
        const row: Record<string, string> = {};
        keys.forEach((k, i) => {
          row[k] = parts[i] ?? "";
        });
        return row;
      });
  }
  return text;
}

export function buildContentPayload(
  schema: ArchetypeSchema,
  raw: Record<string, string>,
): Content {
  const content: Content = {};
  for (const field of schema.fields) {
    if (field.type === "section") continue;
    content[field.name] = parseFieldValue(field, raw[field.name]);
  }
  return content;
}

function isEmptyValue(value: ContentValue): boolean {
  if (typeof value === "string") return value.trim() === "";
  return value.length === 0;
}

/**
 * Client-side required-field check. This is not a substitute for the
 * engine's own `validate()` -- the engine stays authoritative -- it just
 * means the common failure (a blank required field) is bound to its own
 * control immediately instead of after a round trip.
 */
export function validateRequired(
  schema: ArchetypeSchema,
  raw: Record<string, string>,
): FieldError[] {
  const content = buildContentPayload(schema, raw);
  return schema.fields
    .filter((f) => f.required && f.type !== "section")
    .filter((f) => isEmptyValue(content[f.name] ?? ""))
    .map((f) => ({ field: f.name, message: `${f.label || f.name} is required` }));
}

/**
 * The engine returns `ValidationResult.errors` as plain strings today
 * (`"missing required field: to"`) rather than per-field records, so an
 * error array can arrive in either shape. Normalize both, and keep an
 * error whose field cannot be identified rather than dropping it -- an
 * unbindable error still has to reach the creator somewhere.
 */
export function normalizeErrors(
  errors: FieldError[] | string[] | undefined,
  schema?: ArchetypeSchema,
): FieldError[] {
  if (!errors || errors.length === 0) return [];
  const names = new Set((schema?.fields ?? []).map((f) => f.name));
  const list: Array<FieldError | string> = errors;
  return list.map((e) => {
    if (typeof e !== "string") return e;
    const match = /missing required field:\s*(\S+)/i.exec(e);
    const field = match && names.has(match[1]) ? match[1] : "";
    return { field, message: e };
  });
}

export function errorsByField(errors: FieldError[]): Record<string, string> {
  const map: Record<string, string> = {};
  for (const e of errors) {
    if (e.field && !map[e.field]) map[e.field] = e.message;
  }
  return map;
}

/** Placeholder text that teaches the line format a list-shaped field expects. */
export function placeholderFor(field: FieldDef): string {
  if (field.type === "list") return "One item per line";
  if (field.type === "reasoned-list" || field.type === "table-list") {
    const keys = field.keys && field.keys.length ? field.keys : ["a", "b"];
    return `One row per line: ${keys.join(" | ")}`;
  }
  return "";
}
