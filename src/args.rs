use clap::{Args, Parser, Subcommand};
use crate::commands;

/// Pngme CLI
#[derive(Debug, Parser)]
#[clap(name = "pngme")]
#[clap(about = "A CLI to hide messages in PNG files ✨")]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand,)]
pub enum Commands {
    /// Encodes message in png file
    #[clap(arg_required_else_help = true)]
    Encode {
        #[clap(required = true)]
        file_path: String,
        #[clap(required = true)]
        chunk_type: String,
        #[clap(required = true)]
        message: String,
        #[clap(required = false)]
        output_file: Option<String>,
    },
    /// Get message of chunk_type from png at file_path
    #[clap(arg_required_else_help = true)]
    Decode {
        #[clap(required = true)]
        file_path: String,
        #[clap(required = true)]
        chunk_type: String,
    },
    /// Remove message of chunk_type from png at file_path
    #[clap(arg_required_else_help = true)]
    Remove {
        #[clap(required = true)]
        file_path: String,
        #[clap(required = true)]
        chunk_type: String,
    },
    #[clap(arg_required_else_help = true)]
    Print {
        #[clap(required = true)]
        file_path: String
    },
}