//! archetype + content -> a document node tree. Ported from
//! `scope/lib/generate/doc-builder.js`. No AI call anywhere in this path --
//! that's the whole design, inherited unchanged.

use super::archetype::{Archetype, BuildError};
use super::content::Content;
use super::node_tree::DocumentTree;
use super::registry::{get_archetype, RegistryError};
use std::sync::Arc;

#[derive(Debug)]
pub enum GenerateError {
    Registry(RegistryError),
    Build(BuildError),
}

impl std::fmt::Display for GenerateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateError::Registry(e) => write!(f, "{e}"),
            GenerateError::Build(e) => write!(f, "{e}"),
        }
    }
}
impl std::error::Error for GenerateError {}

/// `namespace` = engagement/project id ("tenant-one") or "_common".
/// `archetype_id` = e.g. "single-page-memo". `content` = plain object
/// matching that archetype's schema. Errors on invalid content -- a
/// document missing a required field fails loudly here, before any
/// renderer runs, per canon §3 point 7.
pub fn build(
    namespace: &str,
    archetype_id: &str,
    content: &Content,
) -> Result<(Arc<dyn Archetype>, DocumentTree), GenerateError> {
    let archetype = get_archetype(namespace, archetype_id).map_err(GenerateError::Registry)?;
    let tree = archetype.build(content).map_err(GenerateError::Build)?;
    Ok((archetype, tree))
}
