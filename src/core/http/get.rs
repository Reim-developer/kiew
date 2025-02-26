use crate::log_stdout;
use anyhow::{anyhow, Ok, Result};
use reqwest::get;
use serde_json::{from_str, to_string_pretty, Value};
use std::io::{stdout, Write};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
use owo_colors::OwoColorize;

/// For JSON format, and customize JSON color ouput
fn format_json_output(json_source: &str) -> Result<(), anyhow::Error> {
    let json_value: Value = from_str(json_source)?;

    let format_json = to_string_pretty(&json_value)?;

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let theme_set = ThemeSet::load_defaults();
    let syntax = syntax_set
        .find_syntax_by_extension("json")
        .ok_or_else(|| anyhow!("Syntax for JSON not found"))?;

    let mut hightlight = HighlightLines::new(
        syntax,
        theme_set
            .themes
            .get("base16-eighties.dark")
            .ok_or_else(|| anyhow!("Could not load hightlight"))?,
    );
    log_stdout!("{}", "────────────────── JSON RESPONSE ──────────────────".green());

    let stdout = stdout();
    let mut handle = stdout.lock();

    for line in LinesWithEndings::from(&format_json) {
        let ranges: Vec<(Style, &str)> = hightlight.highlight_line(line, &syntax_set)?;
        let escaped = as_24_bit_terminal_escaped(&ranges, false);
        writeln!(handle, "{escaped}")?;
    }

    Ok(())
} // fn format_json_output

/// Handles HTTP GET request
///
/// # Errors
/// - `Request fails`
/// - `Get response body fails`
pub async fn get_request(website_url: &str) -> Result<(), anyhow::Error> {
    let response = get(website_url).await?;

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(';').next().unwrap_or("").trim().to_lowercase());

    let response_body = get(website_url).await?.text().await?;

    match content_type.as_deref() {
        Some("application/json") => format_json_output(&response_body)?,
        Some("text/html") => log_stdout!("\n{response_body}"),
        _ => log_stdout!("\n{response_body}"),
    }

    Ok(())
} // fn get_request
