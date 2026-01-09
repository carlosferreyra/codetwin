use crate::config::{Config, struct_md_template};
use crate::discovery;
use crate::drivers;
use crate::drivers::markdown::{generate_file_md, generate_index_md};
use crate::drivers::trait_def::Driver;
/// The "Brain" - SyncEngine and the Loop logic
use crate::ir::*;
use std::fs;
use std::path::PathBuf;

pub struct SyncEngine;

impl SyncEngine {
    pub fn new() -> Self {
        SyncEngine
    }

    pub fn watch(&self) -> Result<(), String> {
        Err("watch: Not implemented yet".to_string())
    }

    pub fn sync(&self) -> Result<(), String> {
        // Load config
        let config = Config::load("codetwin.toml")?;
        println!("📖 Config loaded from codetwin.toml");

        // Discover Rust files
        println!("🔍 Discovering source files...");
        let files = discovery::find_rust_files(&config.source_dirs)?;
        println!("   Found {} Rust files", files.len());

        // Parse each file
        println!("🔨 Parsing Rust code...");
        let mut blueprints: Vec<Blueprint> = Vec::new();

        for file_path in files {
            let source = fs::read_to_string(&file_path)
                .map_err(|e| format!("Failed to read {}: {}", file_path.display(), e))?;

            // Get the appropriate driver for the file
            if let Some(driver) = drivers::get_driver_for_file(&file_path) {
                match driver.parse(&source) {
                    Ok(mut blueprint) => {
                        blueprint.source_path = file_path.clone();
                        if !blueprint.elements.is_empty() {
                            println!(
                                "   ✓ Parsed {} ({} items)",
                                file_path.display(),
                                blueprint.elements.len()
                            );
                            blueprints.push(blueprint);
                        }
                    }
                    Err(e) => {
                        eprintln!("   ⚠ Failed to parse {}: {}", file_path.display(), e);
                    }
                }
            }
        }

        if blueprints.is_empty() {
            return Err("No elements found in any source files".to_string());
        }

        // Create output directory
        let output_dir = PathBuf::from(&config.output_dir);
        println!("📝 Creating {} directory...", config.output_dir);
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output dir: {}", e))?;

        // Group blueprints by folder
        use std::collections::BTreeMap;
        let mut folder_blueprints: BTreeMap<String, Vec<Blueprint>> = BTreeMap::new();

        for blueprint in blueprints.iter().cloned() {
            let folder = blueprint
                .source_path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|f| f.to_str())
                .unwrap_or("root")
                .to_string();

            folder_blueprints
                .entry(folder)
                .or_insert_with(Vec::new)
                .push(blueprint);
        }

        // Generate one .md file per folder
        for (folder, folder_bps) in folder_blueprints.iter() {
            let file_name = format!("{}.md", folder);
            let file_path = output_dir.join(&file_name);

            println!("   → Generating {}", file_name);
            let markdown = generate_file_md(&folder_bps)
                .map_err(|e| format!("Failed to generate markdown: {}", e))?;

            fs::write(&file_path, markdown)
                .map_err(|e| format!("Failed to write {}: {}", file_path.display(), e))?;
        }

        // Generate index with folder dependency graph
        let folders: Vec<&String> = folder_blueprints.keys().collect();
        println!("📝 Generating {} index...", config.main_diagram);
        let index = generate_index_md(&folders.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .map_err(|e| format!("Failed to generate index: {}", e))?;

        let index_path = output_dir.join(&config.main_diagram);
        fs::write(&index_path, index).map_err(|e| format!("Failed to write index: {}", e))?;

        println!(
            "\n✅ Successfully generated documentation in {}/",
            config.output_dir
        );
        println!("   Files created:");
        println!("   - {}", index_path.display());
        for folder in folder_blueprints.keys() {
            println!(
                "   - {}",
                output_dir.join(format!("{}.md", folder)).display()
            );
        }

        Ok(())
    }

    pub fn check(&self) -> Result<(), String> {
        Err("check: Not implemented yet".to_string())
    }

    pub fn init(&self) -> Result<(), String> {
        println!("🚀 Initializing codetwin project...\n");

        // Create default config
        let config = Config::default();

        // Create docs directory
        println!("📁 Creating {} directory...", config.output_dir);
        fs::create_dir_all(&config.output_dir)
            .map_err(|e| format!("Failed to create {}: {}", config.output_dir, e))?;

        // Write config file
        println!("⚙️  Creating codetwin.toml...");
        config.save()?;

        // Write template STRUCT.md
        let struct_path = PathBuf::from(&config.output_dir).join(&config.main_diagram);
        println!("📝 Creating {}...", struct_path.display());
        fs::write(&struct_path, struct_md_template())
            .map_err(|e| format!("Failed to write {}: {}", struct_path.display(), e))?;

        println!("\n✅ Project initialized successfully!\n");
        println!("📖 Next steps:");
        println!("   1. Review codetwin.toml and customize if needed");
        println!("   2. Run 'codetwin sync' to generate documentation");
        println!("   3. Check docs/ for the generated STRUCT.md\n");

        Ok(())
    }

    pub fn list(&self) -> Result<(), String> {
        Err("list: Not implemented yet".to_string())
    }
}
