use std::collections::BTreeMap;

use super::Layout;
use crate::ir::{Blueprint, Element, Visibility};

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
    fn format(&self, blueprints: &[Blueprint]) -> Result<Vec<(String, String)>, String> {
        if blueprints.is_empty() {
            return Err("No elements found to format".to_string());
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

        let modules: Vec<&str> = folder_blueprints.keys().map(|s| s.as_str()).collect();
        let index = generate_index_md(&modules)?;
        outputs.push((self.main_diagram.clone(), index));

        Ok(outputs)
    }
}

pub(crate) fn generate_file_md(blueprints: &[Blueprint]) -> Result<String, String> {
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
        output.push_str("## Classes & Functions\n\n");
        output.push_str("```mermaid\n");
        output.push_str(&mermaid);
        output.push_str("```\n\n");
    }

    Ok(output)
}

pub(crate) fn generate_index_md(modules: &[&str]) -> Result<String, String> {
    let mut output = String::new();

    output.push_str("# Project Architecture\n\n");
    output.push_str("## Module Dependencies\n\n");

    output.push_str("```mermaid\n");
    output.push_str("graph TD\n");
    output.push_str("    main[main.rs]\n");
    output.push_str("    cli[cli.rs]\n");
    output.push_str("    engine[engine.rs]\n");
    output.push_str("    ir[ir.rs]\n");
    output.push_str("    drivers[drivers/]\n");
    output.push_str("    io[io/]\n");
    output.push_str("    discovery[discovery.rs]\n\n");
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
    format!("{}.md", module)
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
        }
    }

    Some(diagram)
}
