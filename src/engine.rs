use crate::config::Config;
use crate::discovery;
use crate::drivers;
/// The "Brain" - SyncEngine and the Loop logic
use crate::ir::*;
use crate::layouts;
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

    /// Generate diagrams/documentation from source code
    /// This is the main unidirectional operation: code → diagrams
    pub fn generate(&self, config: &Config) -> Result<(), String> {
        println!("📖 Config loaded (layout: {})", config.layout);

        // Discover source files
        println!("🔍 Discovering source files...");
        let files = discovery::find_rust_files(&config.source_dirs)?;
        println!("   Found {} Rust files", files.len());

        // Parse each file
        println!("🔨 Parsing code...");
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

        // Get output directory from output_file
        let output_path = PathBuf::from(&config.output_file);
        let output_dir = output_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("docs"));

        println!("📝 Creating output directory: {}", output_dir.display());
        fs::create_dir_all(&output_dir)
            .map_err(|e| format!("Failed to create output dir: {}", e))?;

        // Apply layout
        let layout = layouts::get_layout(&config)?;
        let outputs = layout
            .format(&blueprints)
            .map_err(|e| format!("Failed to format documentation: {}", e))?;

        // Write outputs
        println!("📝 Writing formatted outputs...");
        for (file_name, content) in outputs {
            let file_path = output_dir.join(&file_name);
            fs::write(&file_path, content)
                .map_err(|e| format!("Failed to write {}: {}", file_path.display(), e))?;
            println!("   ✓ {}", file_path.display());
        }

        println!(
            "\n✅ Successfully generated documentation in {}/",
            output_dir.display()
        );

        Ok(())
    }

    pub fn init(&self, force: bool) -> Result<(), String> {
        if !force && std::path::Path::new("codetwin.toml").exists() {
            println!("✓ codetwin.toml already initialized");
            return Ok(());
        }

        println!("🚀 Initializing codetwin project...\n");

        // Create default config
        let config = Config::defaults();

        // Write config file
        println!("⚙️  Creating codetwin.toml...");
        config.save(force)?;

        println!("\n✅ Project initialized successfully!\n");
        println!("📖 Next steps:");
        println!("   1. Review codetwin.toml and customize if needed");
        println!("   2. Run 'ctw gen' to generate diagrams");
        println!(
            "   3. Check {} for the generated documentation\n",
            config.output_file
        );

        Ok(())
    }

    pub fn list(&self) -> Result<(), String> {
        Err("list: Not implemented yet".to_string())
    }
}
