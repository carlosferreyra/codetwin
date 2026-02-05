use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "CodeTwin",
    version = "0.1.0",
    about = "Code → Diagram/Documentation Generator",
    long_about = "CodeTwin (ctw) - Unidirectional code to diagram generator\nhttps://github.com/carlosferreyra/codetwin\n\nHelp developers visually understand repository structure and design patterns."
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
    /// Generate diagrams/documentation from source code [DEFAULT]
    Gen {
        /// Override output file path (e.g., docs/api.md)
        #[arg(long, value_name = "PATH")]
        output: Option<String>,

        /// Override layout: dependency-graph, layered, readme-embedded
        #[arg(long, value_name = "LAYOUT")]
        layout: Option<String>,

        /// Override source directory (can be used multiple times)
        #[arg(long, value_name = "DIR")]
        source: Option<Vec<String>>,

        /// Additional exclude pattern (e.g., **/tests/**)
        #[arg(long, value_name = "PATTERN")]
        exclude: Option<Vec<String>>,

        /// Path to custom layout configuration file (IR-compliant TOML)
        #[arg(long, value_name = "FILE")]
        custom_layout: Option<String>,

        /// Persist flag overrides to codetwin.toml
        #[arg(long)]
        save: bool,
    },

    /// Watch source directories and auto-regenerate on changes
    Watch {
        /// Override output file path
        #[arg(long, value_name = "PATH")]
        output: Option<String>,

        /// Override layout
        #[arg(long, value_name = "LAYOUT")]
        layout: Option<String>,

        /// Override source directory (can be used multiple times)
        #[arg(long, value_name = "DIR")]
        source: Option<Vec<String>>,

        /// Time to wait after last event before regenerating (ms) [default: 300]
        #[arg(long, value_name = "MS", default_value = "300")]
        debounce: u64,

        /// Override exclude pattern (can be used multiple times)
        #[arg(long, value_name = "PATTERN")]
        exclude: Option<Vec<String>>,
    },

    /// Initialize or regenerate codetwin.toml configuration
    Init {
        /// Overwrite existing codetwin.toml
        #[arg(long)]
        force: bool,
    },

    /// Debug helper to inspect what CodeTwin detects
    List,
}
