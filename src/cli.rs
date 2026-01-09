use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "CodeTwin",
    version = "0.1.0",
    about = "Bidirectional Architecture Synchronizer",
    long_about = "CodeTwin (ct) - Bidirectional Architecture Synchronizer\nhttps://github.com/carlosferreyra/codetwin"
)]
pub struct Cli {
    /// Enable detailed logs (e.g., scanning specific files)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Silence all output except errors
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Run as if started in <DIR>
    #[arg(short = 'C', long, value_name = "DIR", global = true)]
    pub cwd: Option<String>,

    /// Output results as structured JSON
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Starts the daemon to keep code & docs in sync [DEFAULT]
    Watch {
        /// Override source directory (Default: auto-detect src/ lib/ app/)
        #[arg(short, long, value_name = "DIR")]
        source: Option<String>,

        /// Override output path (Default: auto-detect docs/ or README.md)
        #[arg(short, long, value_name = "PATH")]
        output: Option<String>,

        /// Override strategy [root, fractal, shadow]
        #[arg(long, value_name = "MODE")]
        strategy: Option<String>,

        /// Time to wait after last event (Default: 300)
        #[arg(short, long, value_name = "MS", default_value = "300")]
        debounce: u64,

        /// Send system notification on sync
        #[arg(long)]
        notify: bool,
    },

    /// Runs the synchronization logic once and exits
    Sync {
        /// Override source directory (Default: auto-detect src/ lib/ app/)
        #[arg(short, long, value_name = "DIR")]
        source: Option<String>,

        /// Override output path (Default: auto-detect docs/ or README.md)
        #[arg(short, long, value_name = "PATH")]
        output: Option<String>,

        /// Override strategy [root, fractal, shadow]
        #[arg(long, value_name = "MODE")]
        strategy: Option<String>,

        /// Show what would change without writing to disk
        #[arg(long)]
        dry_run: bool,

        /// One-way sync: Code -> Docs (Never edit code)
        #[arg(long)]
        docs_only: bool,

        /// One-way sync: Docs -> Code (Never edit docs)
        #[arg(long)]
        code_only: bool,

        /// Overwrite files even if conflicting changes detected
        #[arg(short, long)]
        force: bool,
    },

    /// Read-only mode for CI/CD. Returns exit code 1 on drift
    Check {
        /// Fail on minor warnings (missing docstrings, etc.)
        #[arg(long)]
        strict: bool,

        /// Print a unified diff of the drift
        #[arg(long)]
        diff: bool,

        /// Treat warnings as fatal errors
        #[arg(long)]
        fail_on_warnings: bool,
    },

    /// Scaffolds a new configuration or folder structure
    Init {
        /// Create the docs/ folder structure
        #[arg(long)]
        shadow: bool,

        /// Create README.md files in source subdirectories
        #[arg(long)]
        fractal: bool,

        /// Install a pre-commit hook
        #[arg(long)]
        git_hook: bool,
    },

    /// Debug helper to inspect what CodeTwin detects
    List,
}
