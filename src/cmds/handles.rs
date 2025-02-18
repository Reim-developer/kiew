use clap::Parser;

use crate::{
    cli::{CommandLineInterface, Commands},
    core::element::element_count,
};


/// Handles all CLI commmands
pub async fn handles_commands() {
    let args = CommandLineInterface::parse();

    match args.commands {
        Commands::Match { website, element } => {
            match element_count(&website, &element).await {
                Ok(()) => {},
                Err(error) => eprintln!("Get element of website error: {}", error),
            }
        }
    }
}
