mod bundled;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about = "Scribe Papyrus - Papyrus Script Compiler Wrapper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to config file (searches parent dirs by default)
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a new scribe project
    Init,

    /// Compile Papyrus script file(s)
    Compile {
        /// Path to .psc file or directory (use . for sourceDir)
        path: PathBuf,

        /// Check syntax only without compilation
        #[arg(long)]
        check: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => commands::init::run()?,
        Commands::Compile { path, check } => commands::compile::run(path, check, cli.config)?,
    }

    Ok(())
}
