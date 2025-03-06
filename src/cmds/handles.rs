use anyhow::anyhow;
use clap::Parser;

use crate::{
    cli::{CommandLineInterface, Commands}, core::http::{
        delete::delete_request, get::{match_options_get, match_setting_get}, post::post_request, put::put_request,
    }, ultis::setting::generate_setting
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
            details,
            read_setting,
        } => {
            if let (Some(web), Some(ref setting)) = (&website_url, &read_setting) {
                if !web.is_empty() && !setting.is_empty() {
                    return Err(anyhow!("Cannot use '-S' or '--web' argument one time"));
                }
            } 

            if let Some(ref web_url) = website_url {
                if web_url.is_empty() {
                    return Err(anyhow!("Website URL is empty"));
                } else if !web_url.is_ascii() {
                    return Err(anyhow!("Website URL is not ASCII format"))
                }
                match_options_get(web_url, &headers, debug_option, details).await?;
            }

            if let Some(settings) = read_setting {
                if settings.is_empty() {
                    return Err(anyhow!("Website URL is empty"));
                } else if !settings.is_ascii() {
                    return Err(anyhow!("Website URL is not ASCII format"))
                }

                match_setting_get(&settings)?;
            }

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
