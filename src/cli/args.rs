use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new config sync repository
    Init {
        /// The URL of the git repository to clone
        #[arg(short, long)]
        url: Option<String>,
    },
    /// Add a file or directory to be managed
    Add {
        /// The path to the file or directory to add
        path: PathBuf,
    },
    /// Push changes to the remote repository
    Push,
    /// Pull changes from the remote repository and apply them
    Pull,
    /// Watch for changes and sync automatically (daemon mode)
    Watch,
}
