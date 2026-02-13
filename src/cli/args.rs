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
        /// The role(s) of this machine (e.g. "work", "personal")
        #[arg(long)]
        role: Vec<String>,
    },
    /// Add a file or directory to be managed
    Add {
        /// The path to the file or directory to add
        path: PathBuf,
        /// The role(s) this file belongs to
        #[arg(long)]
        role: Vec<String>,
    },
    /// Push changes to the remote repository
    Push,
    /// Pull changes from the remote repository and apply them
    Pull,
    /// Watch for changes and sync automatically (daemon mode)
    Watch,
    /// Manage secrets (encrypted files)
    Secrets {
        #[command(subcommand)]
        command: SecretCommands,
    },
    /// Show commit history
    History,
    /// Undo the last change (revert commit)
    Undo {
        /// Optional commit hash to revert (defaults to HEAD)
        commit: Option<String>,
    },
    /// Diagnose issues with the setup
    Doctor,
}

#[derive(Subcommand, Debug)]
pub enum SecretCommands {
    /// Initialize secrets (generate key pair)
    Init,
    /// Add a secret file (encrypts and adds to repo)
    Add {
        /// Path to the secret file
        path: PathBuf,
    },
}
