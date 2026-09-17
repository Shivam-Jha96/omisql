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
    },
}
