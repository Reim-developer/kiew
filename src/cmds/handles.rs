use clap::Parser;

use crate::{
    cli::{CommandLineInterface, Commands, OptionsScraping::Href},
    colors::LogLevel,
    core::{crawl_href::href_scraper, element::element_count, find_element::find_element},
};

/// Handles all CLI commmands
pub async fn handles_commands() {
    let args = CommandLineInterface::parse();
    let error_color = LogLevel::Error.fmt();

    match args.commands {
        Commands::Match { website, element } => match element_count(&website, &element).await {
            Ok(()) => {}
            Err(error) => eprintln!("{error_color} Fatal error with status: {error}"),
        },

        Commands::Find {
            website,
            element,
            debug_mode,
            log_type,
        } => match find_element(&website, &element, debug_mode, &log_type).await {
            Ok(()) => {}
            Err(error) => eprintln!("{error_color} Fatal error with status: {error}"),
        },

        Commands::Crawl { options } => match options {
            Href { website_url, debug } => {
                if let Err(error) = href_scraper(&website_url,&debug).await {
                    eprintln!("{error_color} Fatal: {error}");
                }
            }
        },
    }
}
