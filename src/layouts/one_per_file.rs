use super::Layout;
use crate::ir::Blueprint;

pub struct OnePerFileLayout;

impl Default for OnePerFileLayout {
    fn default() -> Self {
        OnePerFileLayout
    }
}

impl OnePerFileLayout {
    pub fn new() -> Self {
        OnePerFileLayout
    }
}

impl Layout for OnePerFileLayout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>, String> {
        let mut outputs = Vec::new();

        for blueprint in blueprints {
            let file_stem = blueprint
                .source_path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("file");

            let file_name = format!("{}.md", file_stem);
            let content = format!("# {}\n\n_Not implemented yet.\n", file_stem);
            outputs.push((file_name, content));
        }

        Ok(outputs)
    }
}
