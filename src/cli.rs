use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct CommandLineInterface {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "match")]
    Match {
        #[arg(short = 'w', long = "web")]
        website: String,
        #[arg(short = 'e', long = "element")]
        element: String,
    },
    #[command(name = "find")]
    Find {
        #[arg(short = 'w', long = "web")]
        website: String,
        #[arg(short = 'e', long = "element")]
        element: String,
    },
}
