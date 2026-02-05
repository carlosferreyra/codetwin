use super::trait_def::Layout;
use crate::ir::{Blueprint, Element};
use petgraph::graph::DiGraph;
use petgraph::algo::is_cyclic_directed;
use std::collections::HashMap;

pub struct DependencyGraphLayout;

impl Layout for DependencyGraphLayout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>, String> {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();

        // Build graph: one node per module
        for blueprint in blueprints {
            let module_name = extract_module_name(&blueprint.source_path);
            if !node_indices.contains_key(&module_name) {
                let idx = graph.add_node(module_name.clone());
                node_indices.insert(module_name, idx);
            }
        }

        // Add edges for dependencies
        for blueprint in blueprints {
            let module_name = extract_module_name(&blueprint.source_path);
            if let Some(&source_idx) = node_indices.get(&module_name) {
                for dep in &blueprint.dependencies {
                    if let Some(&target_idx) = node_indices.get(dep) {
                        graph.add_edge(source_idx, target_idx, ());
                    }
                }
            }
        }

        // Check for cycles
        let has_cycles = is_cyclic_directed(&graph);

        // Generate Mermaid diagram
        let mermaid_diagram = generate_mermaid_diagram(&graph, &node_indices, has_cycles);

        // Generate module list with descriptions
        let module_list = generate_module_list(blueprints);

        // Generate markdown output
        let content = format!(
            "{}\n\n{}\n\n{}",
            mermaid_diagram, module_list, generate_footer(has_cycles)
        );

        Ok(vec![("architecture.md".to_string(), content)])
    }
}

/// Extract module name from file path (e.g., "src/engine.rs" -> "engine")
fn extract_module_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Generate Mermaid directed graph diagram
fn generate_mermaid_diagram(
    graph: &DiGraph<String, ()>,
    node_indices: &HashMap<String, petgraph::graph::NodeIndex>,
    has_cycles: bool,
) -> String {
    let mut diagram = String::from("## Dependency Graph\n\n```mermaid\ngraph TD\n");

    // Add nodes with styling
    for (name, _idx) in node_indices {
        if has_cycles {
            diagram.push_str(&format!("    {}[{}]\n", sanitize_id(name), name));
        } else {
            diagram.push_str(&format!("    {}[{}]\n", sanitize_id(name), name));
        }
    }

    // Add edges
    for edge in graph.raw_edges() {
        let from_name = &graph[edge.source()];
        let to_name = &graph[edge.target()];
        diagram.push_str(&format!(
            "    {} --> {}\n",
            sanitize_id(from_name),
            sanitize_id(to_name)
        ));
    }

    diagram.push_str("```\n");
    diagram
}

/// Sanitize module names for Mermaid (replace special chars)
fn sanitize_id(name: &str) -> String {
    name.replace("-", "_")
        .replace(".", "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
}

/// Generate markdown list of modules with their descriptions
fn generate_module_list(blueprints: &[Blueprint]) -> String {
    let mut list = String::from("## Modules\n\n");

    for blueprint in blueprints {
        let module_name = extract_module_name(&blueprint.source_path);
        list.push_str(&format!("### `{}`\n\n", module_name));

        // Add file path
        list.push_str(&format!(
            "**File**: {}\n\n",
            blueprint.source_path.display()
        ));

        // Add elements count
        let class_count = blueprint
            .elements
            .iter()
            .filter(|e| matches!(e, Element::Class(_)))
            .count();
        let function_count = blueprint
            .elements
            .iter()
            .filter(|e| matches!(e, Element::Function(_)))
            .count();

        list.push_str(&format!(
            "**Contents**: {} structs, {} functions\n\n",
            class_count, function_count
        ));

        // Add elements summary
        if !blueprint.elements.is_empty() {
            list.push_str("**Key Types and Functions**:\n\n");
            for element in &blueprint.elements {
                match element {
                    Element::Class(class) => {
                        list.push_str(&format!("- `{}` (struct)\n", class.name));
                    }
                    Element::Function(func) => {
                        list.push_str(&format!("- `{}()` (function)\n", func.name));
                    }
                    Element::Module(_) => {}
                }
            }
            list.push_str("\n");
        }

        // Add dependencies
        if !blueprint.dependencies.is_empty() {
            list.push_str("**Dependencies**: ");
            list.push_str(&blueprint.dependencies.join(", "));
            list.push_str("\n\n");
        }
    }

    list
}

/// Generate footer with cycle detection warning
fn generate_footer(has_cycles: bool) -> String {
    if has_cycles {
        String::from(
            "⚠️ **Circular Dependencies Detected**\n\n\
            This architecture contains circular dependencies. Consider refactoring to break these cycles.",
        )
    } else {
        String::from("✅ No circular dependencies detected.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extract_module_name() {
        assert_eq!(extract_module_name(&PathBuf::from("src/engine.rs")), "engine");
        assert_eq!(extract_module_name(&PathBuf::from("main.rs")), "main");
        assert_eq!(
            extract_module_name(&PathBuf::from("src/drivers/rust.rs")),
            "rust"
        );
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(sanitize_id("engine"), "engine");
        assert_eq!(sanitize_id("std-lib"), "std_lib");
        assert_eq!(sanitize_id("my.module"), "my_module");
        assert_eq!(sanitize_id("some-module!"), "some_module");
    }

    #[test]
    fn test_dependency_graph_format() {
        let mut blueprints = vec![];

        // Create test blueprints
        let bp1 = Blueprint {
            source_path: PathBuf::from("src/main.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec!["engine".to_string()],
        };

        let bp2 = Blueprint {
            source_path: PathBuf::from("src/engine.rs"),
            language: "rust".to_string(),
            elements: vec![],
            dependencies: vec!["config".to_string()],
        };

        blueprints.push(bp1);
        blueprints.push(bp2);

        let layout = DependencyGraphLayout;
        let result = layout.format(&blueprints).unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0, "architecture.md");

        let content = &result[0].1;
        assert!(content.contains("graph TD"));
        assert!(content.contains("main"));
        assert!(content.contains("engine"));
        assert!(content.contains("config"));
    }

    #[test]
    fn test_no_cycles() {
        let blueprints = vec![
            Blueprint {
                source_path: PathBuf::from("src/a.rs"),
                language: "rust".to_string(),
                elements: vec![],
                dependencies: vec!["b".to_string()],
            },
            Blueprint {
                source_path: PathBuf::from("src/b.rs"),
                language: "rust".to_string(),
                elements: vec![],
                dependencies: vec![],
            },
        ];

        let layout = DependencyGraphLayout;
        let result = layout.format(&blueprints).unwrap();
        let content = &result[0].1;

        assert!(content.contains("No circular dependencies detected"));
    }
}
