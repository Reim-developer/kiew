use crate::{colors::LogLevel::Success, fatal};
use crate::log_stdout;
use anyhow::{anyhow, Ok, Result};
use lazy_static::lazy_static;
use mime::Mime;
use owo_colors::OwoColorize;
use reqwest::{Client, Response};
use std::{
    collections::HashMap, io::{stdout, Write}, time::Instant
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

lazy_static! {
    /// Syntax Set
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();

    /// Theme Set
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();

    /// Client
    static ref CLIENT: Client = Client::new();
}

/// For format XML output
fn set_color_scheme_output(source_code: &str, language_type: &str) -> Result<(), anyhow::Error> {
    let syntax = SYNTAX_SET
        .find_syntax_by_extension(language_type)
        .ok_or_else(|| anyhow!("Could not find syntax for {} in SYNTAX_SET", language_type))?;

    let mut highlighter = HighlightLines::new(
        syntax,
        THEME_SET
            .themes
            .get("base16-eighties.dark")
            .ok_or_else(|| anyhow!("Could not load highlight theme"))?,
    );

    
    let mut buffer = Vec::new();
    let header_bar = format!(
        "────────────────── {} RESPONSE ──────────────────",
        language_type.to_uppercase()
    );
    writeln!(buffer, "{}", header_bar.green())?;

    for line in LinesWithEndings::from(source_code) {
        let ranges: Vec<(Style, &str)> = highlighter.highlight_line(line, &SYNTAX_SET)?;
        let escaped = as_24_bit_terminal_escaped(&ranges, false);

        writeln!(buffer, "{escaped}")?;
    }
    stdout().lock().write_all(&buffer)?;

    Ok(())
} // fn set_color_scheme_output

/// Get content type with response
fn get_content_type(response: &Response) -> Result<String, anyhow::Error> {
    response.headers().get("content-type")
    .and_then(|value| value.to_str().ok())
    .and_then(|str| str.parse::<Mime>().ok()).map(|mime| {
        mime.essence_str().to_string()
    }).ok_or_else(|| anyhow!("Could not get Content-Type in {:?}", response.headers()))
}

/// Handles HTTP GET request
///
/// # Errors
/// - `Request fails`
/// - `Get response body fails`
pub async fn get_request(website_url: &str) -> Result<(), anyhow::Error> {
    let start_time = Instant::now();

    let response = CLIENT.get(website_url).send().await?;
    let content_type = get_content_type(&response)?;
    let response_body = response.text().await?;

    let mut content_types: HashMap<&str, &str> = HashMap::new();
    content_types.insert("application/json", "json");
    content_types.insert("text/html", "html");
    content_types.insert("text/plain", "txt");
    content_types.insert("text/xml", "xml");
    content_types.insert("application/javascript", "js");


    if let Some(ext) = content_types.get(content_type.as_str()) {
        set_color_scheme_output(&response_body, ext)?;
    } else {
        fatal!("Unsupported Content-Type: {}", content_type);
        log_stdout!("\n{}", response_body);
    }

    let end_time = start_time.elapsed();
    log_stdout!("{} Finished in {:.2?}", Success.fmt(), end_time);

    Ok(())
} // fn get_request
