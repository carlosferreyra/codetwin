use crate::core::config::Config;
use crate::core::discovery;
/// The "Brain" - SyncEngine and the Loop logic
use crate::core::ir::*;
use crate::drivers;
use crate::layouts;
use anyhow::{Context, Result, anyhow};
use notify_debouncer_mini::new_debouncer;
use notify_debouncer_mini::notify::RecursiveMode;
use rayon::prelude::*;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::time::Duration;
use tracing::{debug, info, warn};
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

    pub fn watch(&self, config: &Config, debounce_ms: u64) -> Result<()> {
        info!("Starting watch mode with {}ms debounce...", debounce_ms);

        let (tx, rx) = mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_millis(debounce_ms), tx)
            .context("Failed to initialize file watcher")?;

        // Watch all source directories
        for source_dir in &config.source_dirs {
            let path = Path::new(source_dir);
            debouncer
                .watcher()
                .watch(path, RecursiveMode::Recursive)
                .context(format!("Failed to watch directory: {}", source_dir))?;
            debug!("Watching directory: {}", source_dir);
        }

        info!("Watching for changes... (Press Ctrl+C to exit)");

        loop {
            match rx.recv() {
                Ok(_) => {
                    info!("File change detected, regenerating documentation...");
                    // Note: In production, we'd want to clone config and pass json_output=false
                    // For now, we'll call generate with fixed json_output=false and no custom_layout
                    if let Err(e) = self.generate(config, false, None) {
                        warn!("Failed to regenerate: {:#}", e);
                    }
                }
                Err(_) => {
                    debug!("File watcher channel closed");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Generate diagrams/documentation from source code
    /// This is the main unidirectional operation: code → diagrams
    pub fn generate(
        &self,
        config: &Config,
        json_output: bool,
        custom_layout: Option<&str>,
    ) -> Result<()> {
        info!("Config loaded: layout={}", config.layout);

        // Discover source files
        debug!("Discovering source files...");
        let files = discovery::find_source_files(&config.source_dirs, &config.exclude_patterns)?;
        info!("Found {} source files", files.len());

        // Parse each file in parallel
        debug!("Parsing code ({} files in parallel)...", files.len());
        let blueprints: Vec<Blueprint> = files
            .par_iter()
            .filter_map(|file_path| {
                match fs::read_to_string(file_path) {
                    Ok(source) => {
                        // Get the appropriate driver for the file
                        if let Some(driver) = drivers::get_driver_for_file(file_path) {
                            match driver.parse(&source) {
                                Ok(mut blueprint) => {
                                    blueprint.source_path = file_path.clone();
                                    if !blueprint.elements.is_empty() {
                                        debug!(
                                            "Parsed {} successfully ({} elements)",
                                            file_path.display(),
                                            blueprint.elements.len()
                                        );
                                        Some(blueprint)
                                    } else {
                                        None
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to parse {}: {}", file_path.display(), e);
                                    None
                                }
                            }
                        } else {
                            None
                        }
                    }
                    Err(e) => {
                        warn!("Failed to read {}: {}", file_path.display(), e);
                        None
                    }
                }
            })
            .collect();

        if blueprints.is_empty() {
            return Err(anyhow!("No elements found in any source files"));
        }

        // Get output directory from output_file
        let output_path = PathBuf::from(&config.output_file);
        let output_dir = output_path
            .parent()
            .unwrap_or_else(|| std::path::Path::new("docs"));

        debug!("Creating output directory: {}", output_dir.display());
        fs::create_dir_all(output_dir).context("Failed to create output dir")?;
        // Apply layout or output as JSON
        if json_output {
            // Output as JSON - convert blueprints to serializable format
            let json_blueprints: Vec<_> = blueprints
                .iter()
                .map(|b| {
                    json!({
                        "source_path": b.source_path.to_string_lossy(),
                        "language": b.language,
                        "elements": b.elements,
                        "dependencies": b.dependencies,
                    })
                })
                .collect();

            let json_output = json!({
                "blueprints": json_blueprints,
                "config": config,
                "generated_at": chrono::Local::now().to_rfc3339()
            });
            let json_str = serde_json::to_string_pretty(&json_output)
                .context("Failed to serialize to JSON")?;
            println!("{}", json_str);
        } else {
            // Get the layout - either custom or from config
            let layout: Box<dyn layouts::Layout> = if let Some(custom_path) = custom_layout {
                layouts::load_custom_layout(custom_path).context("Failed to load custom layout")?
            } else {
                layouts::get_layout(config)?
            };

            let outputs = layout
                .format(&blueprints)
                .context("Failed to format documentation")?;

            // Write outputs
            debug!("Writing formatted outputs");
            for (file_name, content) in outputs {
                let file_path = output_dir.join(&file_name);
                fs::write(&file_path, content)
                    .context(format!("Failed to write {}", file_path.display()))?;
                debug!("Wrote {}", file_path.display());
            }

            info!(
                "Successfully generated documentation in {}",
                output_dir.display()
            );
        }

        Ok(())
    }

    pub fn init(&self, force: bool) -> Result<()> {
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

    pub fn list(&self) -> Result<()> {
        Err(anyhow!("list: Not implemented yet"))
    }
}
