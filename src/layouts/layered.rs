use super::trait_def::Layout;
use crate::config::Layer;
use crate::ir::{Blueprint, Element};
use anyhow::Result;
use glob::Pattern;
use std::collections::HashMap;

pub struct LayeredLayout {
    pub layers: Vec<Layer>,
}

impl LayeredLayout {
    pub fn new(layers: Vec<Layer>) -> Self {
        LayeredLayout { layers }
    }

    /// Get default layers if none provided
    /// Returns empty list - auto-detection will be used when layers are empty
    /// Users should configure custom layers in codetwin.toml if desired
    pub fn defaults() -> Vec<Layer> {
        vec![]
    }
}

impl Layout for LayeredLayout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>> {
        let layers = if self.layers.is_empty() {
            // Auto-detect layers from directory structure
            auto_detect_layers(blueprints)
        } else {
            self.layers.clone()
        };

        // Assign blueprints to layers
        let mut layer_assignments: HashMap<String, Vec<&Blueprint>> = HashMap::new();
        let mut unassigned = Vec::new();

        for blueprint in blueprints {
            let path_str = blueprint.source_path.to_string_lossy().to_string();
            let mut assigned = false;

            for layer in &layers {
                for pattern_str in &layer.patterns {
                    if matches_pattern(&path_str, pattern_str) {
                        layer_assignments
                            .entry(layer.name.clone())
                            .or_insert_with(Vec::new)
                            .push(blueprint);
                        assigned = true;
                        break;
                    }
                }
                if assigned {
                    break;
                }
            }

            if !assigned {
                unassigned.push(blueprint);
            }
        }

        // Build layer descriptions with dependencies
        let mut layer_descriptions = String::new();
        layer_descriptions.push_str("## Layered Architecture\n\n");
        layer_descriptions
            .push_str("This document shows the architecture organized into logical layers.\n");
        layer_descriptions.push_str("Each layer represents a distinct responsibility area.\n\n");

        for layer in &layers {
            if let Some(layer_blueprints) = layer_assignments.get(&layer.name) {
                if layer_blueprints.is_empty() {
                    continue;
                }

                layer_descriptions.push_str(&format!("### Layer: {}\n\n", layer.name));
                layer_descriptions.push_str(&format!(
                    "**Pattern(s)**: {}\n\n",
                    layer.patterns.join(", ")
                ));

                // List modules in this layer
                layer_descriptions.push_str("**Modules**:\n\n");
                for blueprint in layer_blueprints {
                    let module_name = extract_module_name(&blueprint.source_path);
                    let function_count = blueprint
                        .elements
                        .iter()
                        .filter(|e| matches!(e, Element::Function(_)))
                        .count();
                    let class_count = blueprint
                        .elements
                        .iter()
                        .filter(|e| matches!(e, Element::Class(_)))
                        .count();

                    layer_descriptions.push_str(&format!(
                        "- `{}` ({} structs, {} functions)\n",
                        module_name, class_count, function_count
                    ));

                    // List key items
                    if !blueprint.elements.is_empty() {
                        for element in blueprint.elements.iter().take(3) {
                            match element {
                                Element::Class(class) => {
                                    layer_descriptions.push_str(&format!("  - `{}`\n", class.name));
                                }
                                Element::Function(func) => {
                                    layer_descriptions
                                        .push_str(&format!("  - `{}()`\n", func.name));
                                }
                                Element::Module(_) => {}
                            }
                        }
                    }
                }

                // Show dependencies within this layer
                let internal_deps: Vec<String> = layer_blueprints
                    .iter()
                    .flat_map(|b| b.dependencies.clone())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();

                if !internal_deps.is_empty() {
                    layer_descriptions.push_str(&format!(
                        "\n**Internal Dependencies**: {}\n",
                        internal_deps.join(", ")
                    ));
                }

                layer_descriptions.push_str("\n");
            }
        }

        // Add unassigned blueprints
        if !unassigned.is_empty() {
            layer_descriptions.push_str("### Unassigned\n\n");
            layer_descriptions.push_str("Files not explicitly matched to a layer:\n\n");
            for blueprint in &unassigned {
                let module_name = extract_module_name(&blueprint.source_path);
                layer_descriptions.push_str(&format!("- `{}`\n", module_name));
            }
            layer_descriptions.push_str("\n");
        }

        // Generate Mermaid diagram showing layer dependencies
        let mermaid_diagram = generate_layer_diagram(&layers, &layer_assignments, blueprints);
        let content = format!("{}\n\n{}", mermaid_diagram, layer_descriptions);

        Ok(vec![("architecture.md".to_string(), content)])
    }
}

/// Check if a file path matches a glob pattern
/// Auto-detect layers by grouping blueprints by their parent directory
fn auto_detect_layers(blueprints: &[Blueprint]) -> Vec<Layer> {
    let mut layer_map: HashMap<String, Vec<&str>> = HashMap::new();

    for blueprint in blueprints {
        // Get parent directory name
        if let Some(parent) = blueprint.source_path.parent() {
            if let Some(parent_name) = parent.file_name() {
                if let Some(dir_name) = parent_name.to_str() {
                    layer_map
                        .entry(dir_name.to_string())
                        .or_insert_with(Vec::new);
                }
            }
        }
    }

    // Convert to Layer structs, sorted by name for consistency
    let mut layers: Vec<_> = layer_map
        .into_iter()
        .map(|(name, _)| Layer {
            name: name.clone(),
            patterns: vec![format!("{}/**", name)],
        })
        .collect();

    layers.sort_by(|a, b| a.name.cmp(&b.name));
    layers
}

fn matches_pattern(path: &str, pattern_str: &str) -> bool {
    // Simple glob matching
    if let Ok(pattern) = Pattern::new(pattern_str) {
        if pattern.matches(path) {
            return true;
        }
    }

    // Try matching with normalized patterns
    let normalized_pattern = normalize_pattern(pattern_str);
    if let Ok(pattern) = Pattern::new(&normalized_pattern) {
        if pattern.matches(path) {
            return true;
        }
    }

    false
}

/// Normalize glob patterns for consistency
fn normalize_pattern(pattern: &str) -> String {
    // Convert relative paths to glob patterns
    if pattern.contains("**") {
        pattern.to_string()
    } else if pattern.starts_with("src/") {
        pattern.to_string()
    } else {
        format!("src/{}", pattern)
    }
}

/// Extract module name from file path
fn extract_module_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Generate Mermaid diagram showing layers and their inter-dependencies
fn generate_layer_diagram(
    layers: &[Layer],
    layer_assignments: &HashMap<String, Vec<&Blueprint>>,
    _all_blueprints: &[Blueprint],
) -> String {
    let mut diagram = String::from("## Layer Diagram\n\n```mermaid\ngraph TD\n");

    // Add layer boxes
    for layer in layers {
        if layer_assignments.contains_key(&layer.name) {
            diagram.push_str(&format!(
                "    subgraph {} [{}]\n",
                sanitize_id(&layer.name),
                layer.name
            ));
            if let Some(blueprints) = layer_assignments.get(&layer.name) {
                for blueprint in blueprints {
                    let module_name = extract_module_name(&blueprint.source_path);
                    diagram.push_str(&format!(
                        "        {}_{}[{}]\n",
                        sanitize_id(&layer.name),
                        sanitize_id(&module_name),
                        module_name
                    ));
                }
            }
            diagram.push_str("    end\n");
        }
    }

    // Extract inter-layer dependencies
    let mut added_edges = std::collections::HashSet::new();

    for (layer_name, blueprints) in layer_assignments {
        for blueprint in blueprints {
            for dep in &blueprint.dependencies {
                // Find which layer the dependency belongs to
                for (other_layer_name, other_blueprints) in layer_assignments {
                    for other_blueprint in other_blueprints {
                        let other_module = extract_module_name(&other_blueprint.source_path);
                        if dep == &other_module && layer_name != other_layer_name {
                            let edge_key = (layer_name.clone(), other_layer_name.clone());
                            if !added_edges.contains(&edge_key) {
                                diagram.push_str(&format!(
                                    "    {} --> {}\n",
                                    sanitize_id(layer_name),
                                    sanitize_id(other_layer_name)
                                ));
                                added_edges.insert(edge_key);
                            }
                        }
                    }
                }
            }
        }
    }

    diagram.push_str("```\n");
    diagram
}

/// Sanitize names for Mermaid
fn sanitize_id(name: &str) -> String {
    name.replace(" ", "_")
        .replace("-", "_")
        .replace(".", "_")
        .replace("/", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_matches_pattern_simple() {
        assert!(matches_pattern("src/lib.rs", "src/lib.rs"));
        assert!(!matches_pattern("src/main.rs", "src/lib.rs"));
    }

    #[test]
    fn test_matches_pattern_glob() {
        assert!(matches_pattern("src/drivers/rust.rs", "src/drivers/**"));
        assert!(matches_pattern("src/drivers/python.rs", "src/drivers/**"));
        assert!(!matches_pattern("src/main.rs", "src/drivers/**"));
    }

    #[test]
    fn test_extract_module_name() {
        assert_eq!(extract_module_name(&PathBuf::from("src/lib.rs")), "lib");
        assert_eq!(
            extract_module_name(&PathBuf::from("src/engine.rs")),
            "engine"
        );
        assert_eq!(
            extract_module_name(&PathBuf::from("src/drivers/rust.rs")),
            "rust"
        );
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("Core"), "Core");
        assert_eq!(sanitize_id("User Interface"), "User_Interface");
        assert_eq!(sanitize_id("I/O"), "I_O");
    }

    #[test]
    fn test_defaults_empty() {
        // Defaults are now empty - auto-detection is used instead
        let defaults = LayeredLayout::defaults();
        assert!(defaults.is_empty());
    }
}
