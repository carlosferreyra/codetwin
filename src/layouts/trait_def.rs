use crate::ir::Blueprint;
use anyhow::Result;

/// Layout transforms a collection of Blueprints into file outputs.
/// Returns a list of (filename, content) pairs to be written by the engine.
pub trait Layout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>>;
}
