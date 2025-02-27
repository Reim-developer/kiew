use anyhow::anyhow;
use clap::{Parser, Subcommand};
use reqwest::header::{HeaderName, HeaderValue};
use std::str::FromStr;

/// Options for scraping website
#[derive(Subcommand)]
pub enum OptionsScraping {
    /// Find all href in website
    Href {
        /// Argument: Website URL
        /// - Short argument:
        ///- `w` `<WEBSITE_URL>`   
        ///
        /// - Long arugment:
        ///- `web` `<WEBSITE_URL>`
        #[arg(short = 'w', long = "web")]
        website_url: String,

        /// Argument: Debug
        /// - Short argument
        /// - `d` `<OPTION>`
        ///
        /// - Long argument
        /// - `debug` `<OPTIONS>`
        #[arg(short = 'd', long = "debug", default_value = "none")]
        debug: String,
    },
}

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
        #[arg(long = "debug", default_value = "none")]
        debug_mode: String,
    },

    /// Scraper website with customize option
    #[command(name = "crawl")]
    Crawl {
        /// Option for scraper website
        #[command(subcommand)]
        options: OptionsScraping,
    },

    /// GET request command
    #[command(name = "get")]
    Get {
        /// Website URL
        #[arg(short = 'w', long = "web")]
        website_url: String,

        /// Custom headers for request
        #[arg(short = 'H', value_parser = parse_header)]
        headers: Vec<(HeaderName, HeaderValue)>,

        /// Output option
        #[arg(short = 'o', long = "output", default_value = "html")]
        default_output: String,
    },
}

/// Parser header
fn parse_header(s: &str) -> Result<(HeaderName, HeaderValue), anyhow::Error> {
    let parts: Vec<&str> = s.splitn(2, ": ").collect();
    let key_str = parts
        .first()
        .ok_or_else(|| anyhow!("Missing header KEY in {}", s))?;
    let value_str = parts
        .get(1)
        .ok_or_else(|| anyhow!("Missing header VALUE in {}", s))?;

    let key = HeaderName::from_str(key_str)?;
    let value = HeaderValue::from_str(value_str)?;
    Ok((key, value))
}
