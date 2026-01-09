use clap::{CommandFactory, Parser};
use codetwin::cli::{Cli, Commands};
use codetwin::engine::SyncEngine;

fn main() {
    let cli = Cli::parse();

    // Handle global flags
    if cli.verbose {
        eprintln!("[verbose mode enabled]");
    }

    if let Some(cwd) = &cli.cwd {
        if let Err(e) = std::env::set_current_dir(cwd) {
            eprintln!("Error: Failed to change directory to '{}': {}", cwd, e);
            std::process::exit(1);
        }
    }

    let engine = SyncEngine::new();

    let result = match cli.command {
        Some(Commands::Watch { source, output, strategy, debounce, notify }) => {
            if !cli.quiet {
                println!("Starting watch mode...");
                println!("  source: {:?}", source.unwrap_or_else(|| "auto-detect".to_string()));
                println!("  output: {:?}", output.unwrap_or_else(|| "auto-detect".to_string()));
                println!("  strategy: {:?}", strategy.unwrap_or_else(|| "auto-detect".to_string()));
                println!("  debounce: {}ms", debounce);
                println!("  notify: {}", notify);
            }
            engine.watch()
        }
        Some(Commands::Sync { source, output, strategy, dry_run, docs_only, code_only, force }) => {
            if !cli.quiet {
                println!("Running sync...");
                println!("  source: {:?}", source.unwrap_or_else(|| "auto-detect".to_string()));
                println!("  output: {:?}", output.unwrap_or_else(|| "auto-detect".to_string()));
                println!("  strategy: {:?}", strategy.unwrap_or_else(|| "auto-detect".to_string()));
                println!("  dry-run: {}", dry_run);
                println!("  docs-only: {}", docs_only);
                println!("  code-only: {}", code_only);
                println!("  force: {}", force);
            }
            engine.sync()
        }
        Some(Commands::Check { strict, diff, fail_on_warnings }) => {
            if !cli.quiet {
                println!("Running check...");
                println!("  strict: {}", strict);
                println!("  diff: {}", diff);
                println!("  fail-on-warnings: {}", fail_on_warnings);
            }
            engine.check()
        }
        Some(Commands::Init { shadow, fractal, git_hook }) => {
            if !cli.quiet {
                println!("Initializing project structure...");
                println!("  shadow: {}", shadow);
                println!("  fractal: {}", fractal);
                println!("  git-hook: {}", git_hook);
            }
            engine.init()
        }
        Some(Commands::List) => {
            if !cli.quiet {
                println!("Listing detected configuration...");
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
            if !cli.quiet {
                println!("✓ Command completed successfully");
            }
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            std::process::exit(1);
        }
    }
}
