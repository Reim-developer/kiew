use clap::Parser;

use crate::{
    cli::{CommandLineInterface, Commands},
    core::http::{delete::delete_request, get::get_request, post::post_request, put::put_request},
    ultis::setting::generate_setting,
};

/// Handles all CLI commmands
///
/// # Errors
/// Request
/// - `GET` fails
/// - `POST` fails
/// - `DELETE` fails
/// - `PUT` fails
#[inline]
pub async fn handles_commands() -> Result<(), anyhow::Error> {
    let args = CommandLineInterface::parse();

    match args.commands {
        Commands::Get {
            website_url,
            debug_option,
            headers,
        } => {
            get_request(&website_url, &headers, debug_option).await?;
        }
        Commands::Post {
            website_url,
            headers,
            debug_option,
            payload,
        } => {
            post_request(&website_url, &headers, &payload, debug_option).await?;
        }
        Commands::Put {
            website_url,
            headers,
            payload,
            debug_option,
        } => {
            put_request(&website_url, &headers, &payload, debug_option).await?;
        }
        Commands::Delete {
            website_url,
            headers,
            payload,
            debug_option,
        } => {
            delete_request(&website_url, &headers, &payload, debug_option).await?;
        }
        Commands::Setting { file_name } => {
            generate_setting(&file_name)?;
        }
    }

    Ok(())
}
