use clap::{Parser, Subcommand};

/// Define all commands of CLI
#[derive(Parser)]
pub struct CommandLineInterface {
    /// Define sub-commands.
    #[command(subcommand)]
    pub commands: Commands,
}

/// Define all commands
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

        /// Is debug mode enabled?
        #[arg(short = 'd', long = "debug", default_value = "false")]
        debug_mode: bool,
        
        /// For custom log type. such as JSON, Markdown, HTML.
        /// Default as text (*.txt) file
        #[arg(short = 'l', long = "logtype", default_value = "txt")]
        log_type: String
    },
}
