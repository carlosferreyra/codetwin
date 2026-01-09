use crate::ir::Blueprint;

/// Formatter transforms a collection of Blueprints into file outputs.
/// Returns a list of (filename, content) pairs to be written by the engine.
pub trait Formatter {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>, String>;
}
