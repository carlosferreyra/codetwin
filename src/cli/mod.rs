//! Command-line interface.
//!
//! Argument parsing is done with `clap`'s derive API. Each subcommand has its
//! own handler module to keep this file navigable. The `dispatch` function
//! routes a parsed [`Command`] to the right handler.

mod diff;
mod generate;
mod init;
mod list;
mod snapshot;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

/// Top-level CLI entrypoint.
///
/// Global flags (`--verbose`, `--quiet`, `-C`, `--json`) are available on
/// every subcommand.
#[derive(Debug, Parser)]
#[command(
    name = "codetwin",
    version = env!("CARGO_PKG_VERSION"),
    about = "Zero-config code → visual documentation generator",
    long_about = "CodeTwin turns any git repository into high-quality visual documentation.\nhttps://github.com/carlosferreyra/codetwin"
)]
pub struct Cli {
    /// Increase log verbosity (`info` → `debug`).
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Silence non-error output.
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Run as if `codetwin` had been started in `<DIR>` (like `git -C`).
    #[arg(short = 'C', long, value_name = "DIR", global = true)]
    pub cwd: Option<String>,

    /// Emit structured JSON where applicable.
    #[arg(long, global = true)]
    pub json: bool,

    /// Subcommand (defaults to `gen`).
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// All supported subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate documentation (default).
    Gen(GenArgs),

    /// Write or refresh `codetwin.toml`.
    Init(InitArgs),

    /// Capture a `CodeModel` snapshot to `.codetwin/snapshots/`.
    Snapshot(SnapshotArgs),

    /// Diff two snapshots and produce a human-readable report.
    Diff(DiffArgs),

    /// List detected drivers and available layouts.
    List(ListArgs),
}

/// Arguments for `codetwin gen`.
#[derive(Debug, Args, Default, Clone)]
pub struct GenArgs {
    /// Layout to render (`project-overview`, `architecture-map`, ...).
    #[arg(long, value_name = "NAME")]
    pub layout: Option<String>,

    /// Output file path (falls back to `codetwin.toml`).
    #[arg(long, value_name = "PATH")]
    pub output: Option<String>,

    /// Output format (`markdown`, `html`).
    #[arg(long, value_name = "FMT")]
    pub format: Option<String>,

    /// Override source directories (repeatable).
    #[arg(long, value_name = "DIR")]
    pub source: Vec<String>,

    /// Additional exclude patterns (repeatable).
    #[arg(long, value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// Override auto-detected drivers (repeatable).
    #[arg(long, value_name = "NAME")]
    pub drivers: Vec<String>,

    /// Dump the merged `CodeModel` as JSON instead of rendering.
    #[arg(long)]
    pub dump_ir: bool,

    /// Emit one file per module/layer instead of a single file.
    #[arg(long)]
    pub multi_file: bool,

    /// Re-run on filesystem changes.
    #[arg(long)]
    pub watch: bool,

    /// Persist flag values to `codetwin.toml`.
    #[arg(long)]
    pub save: bool,
}

/// Arguments for `codetwin init`.
#[derive(Debug, Args)]
pub struct InitArgs {
    /// Overwrite an existing `codetwin.toml`.
    #[arg(long)]
    pub force: bool,
}

/// Arguments for `codetwin snapshot`.
#[derive(Debug, Args)]
pub struct SnapshotArgs {
    /// Git ref to snapshot (default: `HEAD` / working tree).
    #[arg(long, value_name = "COMMIT")]
    pub r#ref: Option<String>,

    /// Re-snapshot on filesystem changes.
    #[arg(long)]
    pub watch: bool,
}

/// Arguments for `codetwin diff`.
#[derive(Debug, Args)]
pub struct DiffArgs {
    /// Baseline ref (default: last snapshot).
    #[arg(value_name = "REF_A")]
    pub ref_a: Option<String>,

    /// Comparison ref (default: working tree).
    #[arg(value_name = "REF_B")]
    pub ref_b: Option<String>,

    /// Re-diff on filesystem changes.
    #[arg(long)]
    pub watch: bool,
}

/// Arguments for `codetwin list`.
#[derive(Debug, Args)]
pub struct ListArgs {
    /// List detected language drivers.
    #[arg(long)]
    pub drivers: bool,

    /// List registered layouts.
    #[arg(long)]
    pub layouts: bool,
}

/// Route a parsed [`Command`] to the appropriate handler.
///
/// `json` is the global `--json` flag — propagated here so each handler can
/// decide whether to produce machine-readable output.
pub fn dispatch(command: Command, json: bool) -> Result<()> {
    match command {
        Command::Gen(args) => generate::run(args, json),
        Command::Init(args) => init::run(args),
        Command::Snapshot(args) => snapshot::run(args, json),
        Command::Diff(args) => diff::run(args, json),
        Command::List(args) => list::run(args, json),
    }
}
