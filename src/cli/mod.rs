use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Lint SQL files
    Lint {
        /// The SQL file to lint
        path: String,
        
        /// Optional path to a DDL schema file
        #[arg(short, long)]
        schema: Option<String>,

        /// Optional path to a dbt manifest.json file
        #[arg(long)]
        dbt_manifest: Option<String>,

        /// Optional path to a WASM plugin
        #[arg(short, long)]
        plugin: Option<String>,
        
        /// Automatically fix style issues
        #[arg(long)]
        fix: bool,
    },
    /// Start the Language Server Protocol (LSP) server
    Lsp {
        /// Optional path to a DDL schema file
        #[arg(short, long)]
        schema: Option<String>,
    },
}
