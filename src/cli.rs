use clap::{Parser, Subcommand};

/// Define all commands of CLI
#[derive(Parser)]
pub struct CommandLineInterface {
    /// Define sub-commands.
    #[command(subcommand)]
    pub commands: Commands,
}

/// s
#[derive(Subcommand)]
pub enum Commands {
    /// Match command
    #[command(name = "match")]
    Match {
        /// This website target
        #[arg(short = 'w', long = "web")]
        website: String,
        #[arg(short = 'e', long = "element")]
        /// Element you want match
        element: String,
    },

    /// Find command
    #[command(name = "find")]
    Find {
        /// This website target
        #[arg(short = 'w', long = "web")]
        website: String,
        /// This element you want match
        #[arg(short = 'e', long = "element")]
        element: String,
    },
}
