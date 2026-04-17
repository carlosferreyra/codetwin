//! CodeTwin binary entrypoint.
//!
//! Keep this file thin: parse the CLI, configure logging, and dispatch to a
//! subcommand handler. All real logic lives in library modules so it can be
//! covered by integration tests.

use anyhow::Result;
use clap::Parser;
use codetwin::cli::{Cli, Command, dispatch};

fn main() {
    if let Err(err) = run() {
        tracing::error!(error = ?err, "codetwin failed");
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    init_tracing(cli.verbose, cli.quiet);

    if let Some(cwd) = cli.cwd.as_deref() {
        std::env::set_current_dir(cwd)?;
    }

    // Default subcommand is `gen` — match the behaviour of modern CLIs (e.g.
    // `cargo` without a subcommand still does something sensible).
    let command = cli.command.unwrap_or(Command::Gen(Default::default()));
    dispatch(command, cli.json)
}

fn init_tracing(verbose: bool, quiet: bool) {
    use tracing_subscriber::{EnvFilter, fmt};

    let default_level = if quiet {
        "warn"
    } else if verbose {
        "debug"
    } else {
        "info"
    };

    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_writer(std::io::stderr)
        .init();
}
