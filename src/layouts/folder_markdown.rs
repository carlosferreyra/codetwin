use std::collections::BTreeMap;

use super::Layout;
use crate::core::ir::{Blueprint, Element, Visibility};
use crate::drivers;
use anyhow::{Result, anyhow};

pub struct FolderMarkdownLayout {
    main_diagram: String,
}

impl FolderMarkdownLayout {
    pub fn new(main_diagram: impl Into<String>) -> Self {
        FolderMarkdownLayout {
            main_diagram: main_diagram.into(),
        }
    }
}

impl Layout for FolderMarkdownLayout {
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>> {
        if blueprints.is_empty() {
            return Err(anyhow!("No elements found to format"));
        }

        let mut folder_blueprints: BTreeMap<String, Vec<Blueprint>> = BTreeMap::new();

        for blueprint in blueprints.iter().cloned() {
            let folder = blueprint
                .source_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|f| f.to_str())
                .unwrap_or("root")
                .to_string();

            folder_blueprints.entry(folder).or_default().push(blueprint);
        }

        let mut outputs = Vec::new();

        for (folder, folder_bps) in folder_blueprints.iter() {
            let file_name = format!("{}.md", folder);
            let content = generate_file_md(folder_bps)?;
            outputs.push((file_name, content));
        }

        // Generate index dynamically from all blueprints, not hardcoded modules
        let index = generate_index_md(blueprints)?;
        outputs.push((self.main_diagram.clone(), index));

        Ok(outputs)
    }
}

pub(crate) fn generate_file_md(blueprints: &[Blueprint]) -> Result<String> {
    let mut output = String::new();

    if blueprints.is_empty() {
        return Ok(output);
    }

    let folder_name = blueprints[0]
        .source_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|f| f.to_str())
        .unwrap_or("Unknown");

    output.push_str(&format!("# {}\n\n", folder_name));
    output.push_str(&format!("> Language: **{}**\n\n", blueprints[0].language));
    output.push_str("## Files\n\n");
    for bp in blueprints {
        output.push_str(&format!("- `{}`\n", bp.source_path.display()));
    }
    output.push_str("\n---\n\n");

    if let Some(mermaid) = generate_mermaid_diagram_multi(blueprints) {
        let terminology = drivers::terminology_for_language(&blueprints[0].language);
        output.push_str(&format!(
            "## {} & {}\n\n",
            terminology.element_type_plural, terminology.function_label_plural
        ));
        output.push_str("```mermaid\n");
        output.push_str(&mermaid);
        output.push_str("```\n\n");
    }

    Ok(output)
}

pub(crate) fn generate_index_md(blueprints: &[Blueprint]) -> Result<String> {
    let mut output = String::new();

    output.push_str("# Project Architecture\n\n");
    output.push_str("## Module Dependencies\n\n");

    output.push_str("```mermaid\n");
    output.push_str("graph TD\n");

    // Add nodes dynamically from blueprints
    let mut node_ids = std::collections::HashSet::new();
    for blueprint in blueprints {
        let module_name = extract_module_name(&blueprint.source_path);
        let node_id = sanitize_id(&module_name);
        if node_ids.insert(node_id.clone()) {
            output.push_str(&format!("    {}[{}]\n", node_id, module_name));
        }
    }

    // Add edges from dependencies (if available in IR)
    for blueprint in blueprints {
        for dep in &blueprint.dependencies {
            let from_id = sanitize_id(&extract_module_name(&blueprint.source_path));
            let to_id = sanitize_id(dep);
            output.push_str(&format!("    {} --> {}\n", from_id, to_id));
        }
    }

    output.push_str("```\n\n");

    output.push_str("---\n\n");
    output.push_str("## Modules\n\n");

    // Build unique module list from blueprints
    let mut modules_seen = std::collections::HashSet::new();
    for blueprint in blueprints {
        let module_name = extract_module_name(&blueprint.source_path);
        if modules_seen.insert(module_name.clone()) {
            output.push_str(&format!(
                "- [{}]({})\n",
                module_name,
                format_module_path(&module_name)
            ));
        }
    }

    Ok(output)
}

fn format_module_path(module: &str) -> String {
    format!("{}.md", module)
}

/// Extract module name from file path
fn extract_module_name(path: &std::path::Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("unknown")
        .to_string()
}

/// Sanitize names for Mermaid node IDs
fn sanitize_id(name: &str) -> String {
    name.replace(" ", "_")
        .replace("-", "_")
        .replace("/", "_")
        .replace(".", "_")
}

fn mermaid_visibility(vis: &Visibility) -> &'static str {
    match vis {
        Visibility::Public => "+",
        Visibility::Private => "-",
        Visibility::Protected => "#",
        Visibility::Internal => "~",
    }
}

fn generate_mermaid_diagram_multi(blueprints: &[Blueprint]) -> Option<String> {
    let mut diagram = String::from("classDiagram\n");

    for blueprint in blueprints {
        diagram.push_str(&format!(
            "    %% File: {}\n",
            blueprint.source_path.display()
        ));

        for element in &blueprint.elements {
            if let Element::Class(class) = element {
                let terminology = drivers::terminology_for_language(&blueprint.language);
                diagram.push_str(&format!("    class {} {{\n", class.name));

                for prop in &class.properties {
                    let visibility_symbol = mermaid_visibility(&prop.visibility);
                    let type_str = prop.type_annotation.as_deref().unwrap_or("_");
                    diagram.push_str(&format!(
                        "        {}{} {}\n",
                        visibility_symbol, prop.name, type_str
                    ));
                }

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
                    let return_str = method
                        .signature
                        .return_type
                        .as_deref()
                        .unwrap_or(terminology.return_type_default.as_str());

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
        }
    }

    Some(diagram)
}
