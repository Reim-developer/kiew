use clap::Parser;

use crate::{
    cli::CommandLineInterface, cli::Commands,
    colors::LogLevel,
    core::http::{delete::delete_request, get::get_request, post::post_request, put::put_request},
    fatal,
};

/// Handles all CLI commmands
#[inline]
pub async fn handles_commands() {
    let args = CommandLineInterface::parse();
    let error_color = LogLevel::Error.fmt();

    match args.commands {
        Commands::Get {
            website_url,
            debug_option,
            headers,
        } => {
            if let Err(error) = get_request(&website_url, &headers, debug_option).await {
                fatal!("{error_color} Fatal: {error}");
            }
        }
        Commands::Post {
            website_url,
            headers,
            debug_option,
            payload,
        } => {
            if let Err(error) = post_request(&website_url, &headers, &payload, debug_option).await {
                fatal!("{error_color} Fatal: {error}");
            }
        }
        Commands::Put {
            website_url,
            headers,
            payload,
            debug_option,
        } => {
            if let Err(error) = put_request(&website_url, &headers, &payload, debug_option).await {
                fatal!("{error_color} Fatal: {error}");
            }
        }
        Commands::Delete {
            website_url,
            headers,
            payload,
            debug_option,
        } => {
            if let Err(error) = delete_request(&website_url, &headers, &payload, debug_option).await
            {
                fatal!("{error_color} Fatel: {error}");
            }
        }
    }
}
