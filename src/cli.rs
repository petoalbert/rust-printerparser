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
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,

        /// Desired value for `name`
        value: String,
    },

    /// Get username from the DB
    GetName {
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,
    },

    /// Create a checkpoint with the current contents of the file
    Commit {
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,

        /// Path to the blender file to create the commit from
        #[arg(short, long)]
        file_path: String,

        /// A short summary of the changes
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Write the contents of a checkpoint to a file
    Restore {
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,

        /// Path of the file to write to
        #[arg(short, long)]
        file_path: String,

        /// The hash of the comit to check out
        #[arg(long)]
        hash: String,
    },

    /// List the checkpoints so far
    Log {
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,
    },

    /// Create a new branch
    NewBranch {
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,

        /// The name of the new branch
        #[arg(short, long)]
        branch_name: String,
    },

    /// Lists all existing branches
    ListBranches {
        /// Path to the blend file DB
        #[arg(short, long)]
        db_path: String,
    },
}

pub fn parse_args() -> Cli {
    Cli::parse()
}
