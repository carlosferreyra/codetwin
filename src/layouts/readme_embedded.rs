use super::trait_def::Layout;
use crate::core::ir::{Blueprint, Element};
use anyhow::Result;
use std::collections::HashMap;

pub struct ReadmeEmbeddedLayout;

impl Layout for ReadmeEmbeddedLayout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>> {
        let mut content = String::new();

        // Title and overview
        content.push_str("## Project Architecture\n\n");
        content.push_str("Quick overview of the codebase structure and key components.\n\n");

        // Component table
        content.push_str(&generate_component_table(blueprints));
        content.push('\n');

        // Mermaid diagram (compact)
        content.push_str(&generate_compact_diagram(blueprints));
        content.push('\n');

        // Data flow explanation
        content.push_str(&generate_data_flow(blueprints));
        content.push('\n');

        // Development guide
        content.push_str(&generate_dev_guide(blueprints));

        Ok(vec![("architecture.md".to_string(), content)])
    }
}

/// Generate component overview table
fn generate_component_table(blueprints: &[Blueprint]) -> String {
    let mut table = String::new();
    table.push_str("### Components\n\n");
    table.push_str("| Module | Purpose | Key Functions |\n");
    table.push_str("|--------|---------|----------------|\n");

    for blueprint in blueprints {
        let module_name = extract_module_name(&blueprint.source_path);

        // Extract purpose from documentation or infer from functions
        let purpose = infer_purpose(&blueprint.elements, &module_name);

        // Get key functions (up to 3)
        let key_functions = extract_key_functions(&blueprint.elements, 3);

        table.push_str(&format!(
            "| `{}` | {} | {} |\n",
            module_name, purpose, key_functions
        ));
    }

    table
}

/// Infer module purpose from its elements and documentation
fn infer_purpose(elements: &[Element], _module_name: &str) -> String {
    // Try to get documentation first
    if let Some(doc) = elements.iter().find_map(|e| match e {
        Element::Module(m) => m.documentation.summary.as_ref(),
        Element::Class(c) => c.documentation.summary.as_ref(),
        _ => None,
    }) {
        return doc.chars().take(50).collect();
    }

    // If no documentation, return generic description
    "Module".to_string()
}

/// Extract key functions from blueprint elements
fn extract_key_functions(elements: &[Element], max_count: usize) -> String {
    let functions: Vec<String> = elements
        .iter()
        .filter_map(|e| match e {
            Element::Function(f) => Some(format!("`{}`", f.name)),
            Element::Class(c) => {
                if !c.methods.is_empty() {
                    let method_names: Vec<String> = c
                        .methods
                        .iter()
                        .take(2)
                        .map(|m| format!("`{}`", m.name))
                        .collect();
                    Some(format!("{} ({})", c.name, method_names.join(", ")))
                } else {
                    Some(format!("`{}`", c.name))
                }
            }
            _ => None,
        })
        .take(max_count)
        .collect();

    if functions.is_empty() {
        "—".to_string()
    } else {
        functions.join(", ")
    }
}

/// Generate compact Mermaid dependency diagram
fn generate_compact_diagram(blueprints: &[Blueprint]) -> String {
    let mut diagram = String::new();
    diagram.push_str("### Dependency Overview\n\n");
    diagram.push_str("```mermaid\ngraph TD\n");

    // Build a node-to-deps mapping
    let mut nodes = HashMap::new();
    for blueprint in blueprints {
        let module_name = extract_module_name(&blueprint.source_path);
        nodes.insert(module_name.clone(), blueprint.dependencies.clone());
    }

    // Add nodes
    for module_name in nodes.keys() {
        diagram.push_str(&format!(
            "    {}[{}]\n",
            sanitize_id(module_name),
            module_name
        ));
    }

    // Add edges (only show "important" ones to avoid clutter)
    // Important = either direction has dependencies
    let mut added_edges = std::collections::HashSet::new();
    let importance_threshold = 1;

    for (module_name, deps) in &nodes {
        // Only show edges for modules with dependencies
        if deps.len() >= importance_threshold {
            for dep in deps {
                let edge_key = format!("{}->{}", module_name, dep);
                if !added_edges.contains(&edge_key) && nodes.contains_key(dep) {
                    diagram.push_str(&format!(
                        "    {} --> {}\n",
                        sanitize_id(module_name),
                        sanitize_id(dep)
                    ));
                    added_edges.insert(edge_key);
                }
            }
        }
    }

    diagram.push_str("```\n");
    diagram
}

/// Generate data flow explanation
fn generate_data_flow(blueprints: &[Blueprint]) -> String {
    let mut flow = String::new();
    flow.push_str("### Data Flow\n\n");

    // Infer main entry points
    let entry_points: Vec<String> = blueprints
        .iter()
        .filter(|b| {
            let name = extract_module_name(&b.source_path);
            name == "main" || name == "lib" || name == "cli"
        })
        .map(|b| extract_module_name(&b.source_path))
        .collect();

    let mut step = 1;

    if !entry_points.is_empty() {
        flow.push_str(&format!(
            "{}. **Entry Point** ({}) - User starts the application\n\n",
            step,
            entry_points.join(", ")
        ));
        step += 1;
    }

    // Try to infer processing steps
    let has_discovery = blueprints
        .iter()
        .any(|b| extract_module_name(&b.source_path) == "discovery");
    let has_config = blueprints
        .iter()
        .any(|b| extract_module_name(&b.source_path) == "config");
    let has_engine = blueprints
        .iter()
        .any(|b| extract_module_name(&b.source_path) == "engine");
    let has_drivers = blueprints.iter().any(|b| {
        extract_module_name(&b.source_path).contains("driver")
            || extract_module_name(&b.source_path).contains("parse")
    });
    let has_layouts = blueprints
        .iter()
        .any(|b| extract_module_name(&b.source_path).contains("layout"));

    if has_config {
        flow.push_str(&format!(
            "{}. **Configuration** - Load and parse settings\n\n",
            step
        ));
        step += 1;
    }

    if has_discovery {
        flow.push_str(&format!(
            "{}. **Discovery** - Scan filesystem for source files\n\n",
            step
        ));
        step += 1;
    }

    if has_drivers {
        flow.push_str(&format!(
            "{}. **Parsing** - Extract code structure using language drivers\n\n",
            step
        ));
        step += 1;
    }

    if has_engine {
        flow.push_str(&format!(
            "{}. **Processing** - Transform parsed code into intermediate representation\n\n",
            step
        ));
        step += 1;
    }

    if has_layouts {
        flow.push_str(&format!(
            "{}. **Formatting** - Apply selected layout to generate output\n\n",
            step
        ));
        step += 1;
    }

    flow.push_str(&format!(
        "{}. **Output** - Write generated documentation to file\n\n",
        step
    ));

    flow
}

/// Generate development guide section
fn generate_dev_guide(blueprints: &[Blueprint]) -> String {
    let mut guide = String::new();
    guide.push_str("### Development Guide\n\n");

    guide.push_str("#### Key Files\n\n");

    // Identify and list key files
    for blueprint in blueprints {
        let module_name = extract_module_name(&blueprint.source_path);
        if matches!(module_name.as_str(), "lib" | "main" | "engine" | "config") {
            if let Some(doc) = blueprint.elements.iter().find_map(|e| match e {
                Element::Module(m) => m.documentation.summary.as_ref(),
                _ => None,
            }) {
                guide.push_str(&format!("- `{}` - {}\n", module_name, doc));
            } else {
                guide.push_str(&format!("- `{}` - Core module\n", module_name));
            }
        }
    }

    guide.push_str("\n#### How to Extend\n\n");

    let has_drivers = blueprints
        .iter()
        .any(|b| extract_module_name(&b.source_path).contains("driver"));
    let has_layouts = blueprints
        .iter()
        .any(|b| extract_module_name(&b.source_path).contains("layout"));

    if has_drivers || has_layouts {
        guide.push_str("**This project supports extensions:**\n\n");
        if has_drivers {
            guide.push_str("- Add new language support by implementing the driver trait\n");
        }
        if has_layouts {
            guide.push_str("- Add output formats by implementing the layout trait\n");
        }
        guide.push_str("- Follow the existing code patterns and architecture structure\n\n");
    }

    guide.push_str("**General guidelines:**\n\n");
    guide.push_str("- Follow the existing code style\n");
    guide.push_str("- Add tests for new functionality\n");
    guide.push_str("- Ensure error handling uses `anyhow::Result`\n");
    guide.push_str("- Use structured logging with `tracing` macros\n\n");

    guide
}

/// Extract module name from file path
fn extract_module_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string()
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
    fn test_extract_module_name() {
        assert_eq!(extract_module_name(&PathBuf::from("src/lib.rs")), "lib");
        assert_eq!(
            extract_module_name(&PathBuf::from("src/engine.rs")),
            "engine"
        );
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("MyModule"), "MyModule");
        assert_eq!(sanitize_id("my-module"), "my_module");
        assert_eq!(sanitize_id("my.module"), "my_module");
    }

    #[test]
    fn test_infer_purpose_generic() {
        // Purpose is now generic when no documentation is available
        let purpose = infer_purpose(&[], "cli");
        assert_eq!(purpose, "Module");

        let purpose2 = infer_purpose(&[], "unknown_module");
        assert_eq!(purpose2, "Module");
    }
}
