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

    Commit {
        file_path: String,
    },

    Checkout {
        file_path: String,
        hash: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
