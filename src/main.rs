use clap::{CommandFactory, Parser};
use codetwin::cli::{Cli, Commands};
use codetwin::config::Config;
use codetwin::engine::SyncEngine;
use std::path::Path;

fn main() {
    // Initialize tracing subscriber with env-filter
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Handle global flags
    if cli.verbose {
        eprintln!("[verbose mode enabled]");
    }

    if let Some(cwd) = &cli.cwd
        && let Err(e) = std::env::set_current_dir(cwd)
    {
        eprintln!("Error: Failed to change directory to '{}': {}", cwd, e);
        std::process::exit(1);
    }

    let engine = SyncEngine::new();

    let result: anyhow::Result<()> = match cli.command {
        Some(Commands::Gen {
            output,
            layout,
            source,
            exclude,
            custom_layout,
            save,
        }) => {
            // Load config or create defaults
            let mut config = Config::load_or_defaults("codetwin.toml");

            // Auto-create config if it doesn't exist
            if !Path::new("codetwin.toml").exists() {
                if !cli.quiet {
                    println!("⚙️  Creating codetwin.toml with defaults...");
                }
                if let Err(e) = config.save(false) {
                    eprintln!("Warning: Could not save initial config: {}", e);
                }
            }

            // Apply ephemeral flag overrides
            if let Some(o) = output {
                config.output_file = o;
            }
            if let Some(l) = layout {
                config.layout = l;
            }
            if let Some(s) = source {
                config.source_dirs = s;
            }
            if let Some(e) = exclude {
                config.exclude_patterns.extend(e);
            }

            // Persist if --save flag is set
            if save {
                if !cli.quiet {
                    println!("💾 Saving configuration to codetwin.toml...");
                }
                if let Err(e) = config.save(true) {
                    eprintln!("Error saving config: {}", e);
                }
            }

            engine.generate(&config, cli.json, custom_layout.as_deref())
        }
        Some(Commands::Watch {
            output,
            layout,
            source,
            debounce,
            exclude,
        }) => {
            // Load config or create defaults
            let mut config = Config::load_or_defaults("codetwin.toml");

            // Auto-create config if it doesn't exist
            if !Path::new("codetwin.toml").exists() {
                if !cli.quiet {
                    println!("⚙️  Creating codetwin.toml with defaults...");
                }
                if let Err(e) = config.save(false) {
                    eprintln!("Warning: Could not save initial config: {}", e);
                }
            }

            // Apply ephemeral flag overrides
            if let Some(o) = output {
                config.output_file = o;
            }
            if let Some(l) = layout {
                config.layout = l;
            }
            if let Some(s) = source {
                config.source_dirs = s;
            }
            if let Some(e) = exclude {
                config.exclude_patterns.extend(e);
            }

            if !cli.quiet {
                println!("👀 Watching for changes (debounce: {}ms)...", debounce);
            }
            engine.watch(&config, debounce)
        }
        Some(Commands::Init { force }) => {
            if !cli.quiet {
                println!("🚀 Initializing codetwin...");
            }
            engine.init(force)
        }
        Some(Commands::List) => {
            if !cli.quiet {
                println!("📋 Listing detected configuration...");
            }
            engine.list()
        }
        None => {
            // No subcommand provided - print help and exit
            let mut cmd = Cli::command();
            cmd.print_help().ok();
            println!();
            std::process::exit(0);
        }
    };

    match result {
        Ok(_) => {
            if !cli.quiet && !cli.json {
                println!("✓ Done");
            }
        }
        Err(e) => {
            eprintln!("✗ Error: {:#}", e);
            std::process::exit(1);
        }
    }
}
