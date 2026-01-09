use super::trait_def::Driver;
/// Markdown generator - Blueprint → file-level .rs.md with class diagram
use crate::ir::{Blueprint, Element, Visibility};

pub struct MarkdownDriver;

impl Driver for MarkdownDriver {
    fn parse(&self, _content: &str) -> Result<Blueprint, String> {
        Err("MarkdownDriver::parse: Not implemented yet (Markdown is a target, not a source for now)".to_string())
    }

    fn generate(&self, blueprint: &Blueprint) -> Result<String, String> {
        generate_file_md(blueprint)
    }
}

/// Generate markdown for a single file (.rs.md)
pub fn generate_file_md(blueprint: &Blueprint) -> Result<String, String> {
    let mut output = String::new();

    // Header
    output.push_str(&format!(
        "# {}\n\n",
        blueprint
            .source_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
    ));

    output.push_str(&format!("> Language: **{}**\n\n", blueprint.language));
    output.push_str(&format!(
        "> Source: `{}`\n\n",
        blueprint.source_path.display()
    ));

    output.push_str("---\n\n");

    // Mermaid Class Diagram
    if let Some(mermaid) = generate_mermaid_diagram(blueprint) {
        output.push_str("## Class Diagram\n\n");
        output.push_str("```mermaid\n");
        output.push_str(&mermaid);
        output.push_str("```\n\n");
    }

    Ok(output)
}

/// Generate markdown for the root STRUCT.md (module/folder overview)
pub fn generate_index_md(modules: &[&str]) -> Result<String, String> {
    let mut output = String::new();

    output.push_str("# Project Architecture\n\n");
    output.push_str("## Module Dependencies\n\n");

    // Simple folder graph (can be enhanced with real dependency analysis)
    output.push_str("```mermaid\n");
    output.push_str("graph TD\n");
    output.push_str("    main[main.rs]\n");
    output.push_str("    cli[cli.rs]\n");
    output.push_str("    engine[engine.rs]\n");
    output.push_str("    ir[ir.rs]\n");
    output.push_str("    drivers[drivers/]\n");
    output.push_str("    io[io/]\n");
    output.push_str("    discovery[discovery.rs]\n");
    output.push_str("\n");
    output.push_str("    main --> cli\n");
    output.push_str("    cli --> engine\n");
    output.push_str("    engine --> drivers\n");
    output.push_str("    engine --> ir\n");
    output.push_str("    engine --> io\n");
    output.push_str("    engine --> discovery\n");
    output.push_str("```\n\n");

    output.push_str("---\n\n");
    output.push_str("## Files\n\n");

    for module in modules {
        output.push_str(&format!("- [{}]({})\n", module, format_module_path(module)));
    }

    Ok(output)
}

fn format_module_path(module: &str) -> String {
    format!("src/{}.md", module.replace("::", "/"))
}

fn generate_mermaid_diagram(blueprint: &Blueprint) -> Option<String> {
    // Only generate if there are classes or functions
    if blueprint.elements.is_empty() {
        return None;
    }

    let mut diagram = String::from("classDiagram\n");

    for element in &blueprint.elements {
        match element {
            Element::Class(class) => {
                diagram.push_str(&format!("    class {} {{\n", class.name));

                // Properties
                for prop in &class.properties {
                    let visibility_symbol = mermaid_visibility(&prop.visibility);
                    let type_str = prop.type_annotation.as_deref().unwrap_or("_");
                    diagram.push_str(&format!(
                        "        {}{} {}\n",
                        visibility_symbol, prop.name, type_str
                    ));
                }

                // Methods
                for method in &class.methods {
                    let visibility_symbol = mermaid_visibility(&method.visibility);
                    let params: Vec<String> = method
                        .signature
                        .parameters
                        .iter()
                        .filter(|p| p.name != "self")
                        .map(|p| {
                            let type_str = p.type_annotation.as_deref().unwrap_or("_");
                            format!("{}: {}", p.name, type_str)
                        })
                        .collect();
                    let return_str = method.signature.return_type.as_deref().unwrap_or("void");

                    diagram.push_str(&format!(
                        "        {}{}({}) {}\n",
                        visibility_symbol,
                        method.name,
                        params.join(", "),
                        return_str
                    ));
                }

                diagram.push_str("    }\n");
            }
            Element::Function(_) => {
                // Functions could be represented as utility classes or module-level
                // For now, skip standalone functions in class diagram
            }
            Element::Module(_) => {
                // Modules typically aren't shown in class diagrams
            }
        }
    }

    Some(diagram)
}

fn mermaid_visibility(vis: &Visibility) -> &'static str {
    match vis {
        Visibility::Public => "+",
        Visibility::Private => "-",
        Visibility::Protected => "#",
        Visibility::Internal => "~",
    }
}
