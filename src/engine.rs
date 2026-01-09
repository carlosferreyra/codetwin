use crate::config::{Config, struct_md_template};
use crate::discovery;
use crate::drivers;
use crate::formatters;
/// The "Brain" - SyncEngine and the Loop logic
use crate::ir::*;
use std::fs;
use std::path::PathBuf;

pub struct SyncEngine;

impl Default for SyncEngine {
    fn default() -> Self {
        Self::new()
    }
}

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

        let output_dir = PathBuf::from(&config.output_dir);
        println!("📝 Creating {} directory...", config.output_dir);
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output dir: {}", e))?;

        let formatter = formatters::get_formatter(&config)?;
        let outputs = formatter
            .format(&blueprints)
            .map_err(|e| format!("Failed to format documentation: {}", e))?;

        println!("📝 Writing formatted outputs...");
        for (file_name, content) in outputs {
            let file_path = output_dir.join(&file_name);
            fs::write(&file_path, content)
                .map_err(|e| format!("Failed to write {}: {}", file_path.display(), e))?;
            println!("   - {}", file_path.display());
        }

        println!(
            "\n✅ Successfully generated documentation in {}/",
            config.output_dir
        );

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
