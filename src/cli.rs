use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(about = "The blender version manager tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Test by reading a file and writing it back into another one
    Test {
        /// path of blender file to read
        #[arg(short, long)]
        from_path: String,

        /// path to write to
        #[arg(short, long)]
        to_path: String,
    },

    /// Set username in the DB
    SetName {
        value: String,
    },

    /// Get username from the DB
    GetName,

    /// Create a checkpoint with the current contents of the file
    Commit {
        file_path: String,

        /// A short summary of the changes
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Write the contents of a checkpoint to a file
    Checkout {
        file_path: String,

        /// The hash of the comit to check out
        hash: String,
    },

    /// List the checkpoints so far
    Log,
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
