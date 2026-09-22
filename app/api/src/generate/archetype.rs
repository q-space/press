//! The `Archetype` trait -- a validator + a `build()` function, same shape
//! as a JS archetype module (`{id, title, governance, filenameFields,
//! fields, validate, build}`). Rust has no dynamic `require()` of a
//! directory, so where the JS `registry.js` scans `archetypes/` at
//! runtime, `registry.rs` instead lists each archetype's constructor
//! explicitly at compile time -- the registry's public behavior (look up
//! by namespace + id, `_common` as fallback) is unchanged.

use super::content::Content;
use super::node_tree::DocumentTree;

#[derive(Debug, Clone, Copy)]
pub struct FieldDef {
    pub name: &'static str,
    pub label: &'static str,
    /// "text" | "textarea" | "select" | "list" | "reasoned-list" |
    /// "table-list" | "section" -- kept as a bare string, same as the JS
    /// field vocabulary, rather than an enum, since Qpress's own
    /// Creator-Studio wizard (BB26091204) is the actual consumer of this
    /// value and treats it as data, not Rust-side control flow.
    pub field_type: &'static str,
    pub required: bool,
    /// Groups this field under a named fieldset in the wizard UI (e.g.
    /// formal-letter's "Header"/"Body"/"Closing") -- `None` for a flat
    /// field list.
    pub section: Option<&'static str>,
    /// For `field_type: "reasoned-list"`/`"table-list"` -- the sub-keys
    /// each list row carries (e.g. `["option", "detail"]`).
    pub keys: Option<&'static [&'static str]>,
}

/// A `FieldDef` with only the always-present attributes set -- most fields
/// have no `section`/`keys`, so archetypes build their `FIELDS` array with
/// this as a base (`FieldDef { name: "x", ..FIELD_DEFAULTS }`) rather than
/// repeating `section: None, keys: None` on every entry.
pub const FIELD_DEFAULTS: FieldDef = FieldDef {
    name: "",
    label: "",
    field_type: "text",
    required: false,
    section: None,
    keys: None,
};

#[derive(Debug, Clone, Copy)]
pub struct FilenameFields {
    pub primary: &'static str,
    pub secondary: &'static str,
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct BuildError(pub String);

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for BuildError {}

pub trait Archetype: Send + Sync {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn governance(&self) -> bool {
        false
    }
    fn filename_fields(&self) -> FilenameFields;
    fn fields(&self) -> &'static [FieldDef];
    /// BA26081810's layout-metadata renderer hint (e.g. "header-block") --
    /// most archetypes have none.
    fn layout(&self) -> Option<&'static str> {
        None
    }
    fn validate(&self, content: &Content) -> ValidationResult;
    /// Mirrors doc-builder.js's contract: throws (here, `Err`) on invalid
    /// content rather than building a partial document -- a document
    /// missing a required field fails loudly before any renderer runs.
    fn build(&self, content: &Content) -> Result<DocumentTree, BuildError>;
}
