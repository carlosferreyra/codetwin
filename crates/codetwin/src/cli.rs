//! CLI boundary for repository inspection and legacy compatibility commands.

use std::path::PathBuf;

use anyhow::{Result, bail};
use clap::{Args, Parser, Subcommand};
use codetwin_legacy::cli::{Command as LegacyCommand, GenArgs};

use crate::{analysis, visualization};

/// CodeTwin command-line arguments.
#[derive(Debug, Parser)]
#[command(
    name = "codetwin",
    version,
    about = "Visualize the structure of a repository"
)]
pub struct Cli {
    /// Increase log verbosity.
    #[arg(short, long, global = true)]
    pub verbose: bool,
    /// Silence progress messages.
    #[arg(short, long, global = true)]
    pub quiet: bool,
    /// Resolve repository paths from this directory.
    #[arg(short = 'C', long, global = true, value_name = "DIR")]
    pub cwd: Option<PathBuf>,
    /// Emit structured JSON where applicable.
    #[arg(long, global = true)]
    pub json: bool,
    /// Command; the legacy default remains `gen`.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Developer tasks exposed by the executable.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Inspect packages and declared local dependencies in a Rust workspace.
    Inspect(RepositoryArgs),
    /// Generate a Markdown workspace overview and linked crate details.
    Visualize(VisualizeArgs),
    /// Existing commands retained for compatibility.
    #[command(flatten)]
    Legacy(LegacyCommand),
}

/// Repository selection shared by inspection and visualization.
#[derive(Debug, Args)]
pub struct RepositoryArgs {
    /// Repository directory; defaults to the invocation directory.
    #[arg(default_value = ".")]
    pub repository: PathBuf,
}

/// Visualization destination and repository selection.
#[derive(Debug, Args)]
pub struct VisualizeArgs {
    /// Repository selection.
    #[command(flatten)]
    pub repository: RepositoryArgs,
    /// Output directory relative to the repository, or an absolute path.
    #[arg(long, default_value = "docs/code", value_name = "DIR")]
    pub output: PathBuf,
}

/// Route developer tasks to repository analysis or existing legacy handlers.
pub fn dispatch(cli: Cli) -> Result<()> {
    let command = cli
        .command
        .unwrap_or(Command::Legacy(LegacyCommand::Gen(GenArgs::default())));
    match command {
        Command::Inspect(args) => {
            let base = cli.cwd.unwrap_or(std::env::current_dir()?);
            let analysis = analysis::inspect(&base.join(args.repository))?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&analysis.model)?);
            } else {
                println!(
                    "Rust workspace: {} packages, {} local dependency declarations",
                    analysis.model.packages.len(),
                    analysis.model.package_dependencies.len()
                );
                for package in &analysis.model.packages {
                    println!("{} ({})", package.name, package.manifest_path);
                }
            }
            Ok(())
        }
        Command::Visualize(args) => {
            if cli.json {
                bail!("visualize writes Markdown; use inspect --json for the Code Model");
            }
            let base = cli.cwd.unwrap_or(std::env::current_dir()?);
            let analysis = analysis::inspect(&base.join(args.repository.repository))?;
            let output = analysis.root.join(args.output);
            let artifacts = visualization::render(&analysis.model, &analysis.root, &output)?;
            visualization::write(&output, &artifacts)?;
            if !cli.quiet {
                println!(
                    "Wrote {} Markdown files to {}",
                    artifacts.len(),
                    output.display()
                );
            }
            Ok(())
        }
        Command::Legacy(command) => {
            if let Some(cwd) = cli.cwd {
                std::env::set_current_dir(cwd)?;
            }
            codetwin_legacy::cli::dispatch(command, cli.json)
        }
    }
}
