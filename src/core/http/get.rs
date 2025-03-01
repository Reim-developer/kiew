use crate::log_stdout;
use crate::{colors::LogLevel::Info, colors::LogLevel::Success, fatal};
use anyhow::{anyhow, Ok, Result};
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use mime::Mime;
use owo_colors::OwoColorize;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Response};
use std::fs::File;
use std::path::Path;
use std::time::Duration;
use std::{
    collections::HashMap,
    io::{stdout, Write},
    time::Instant,
};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};
use url::Url;

lazy_static! {
    /// Syntax Set
    static ref SYNTAX_SET: SyntaxSet = SyntaxSet::load_defaults_newlines();

    /// Theme Set
    static ref THEME_SET: ThemeSet = ThemeSet::load_defaults();

    /// Client
    static ref CLIENT: Client = Client::new();
}

/// Save log format
fn save_as(website_url: &str, ext: &str, source: &str, debug: bool) -> Result<bool, anyhow::Error> {
    if debug {
        let url = Url::parse(website_url)?;
        let web_domain = url
            .host_str()
            .ok_or_else(|| anyhow!("Could not get website domain"))?;
        let mut default_file_name = format!("{web_domain}.{ext}");

        let mut number: i32 = 0;
        while Path::new(&default_file_name).exists() {
            number = number.saturating_add(1);
            default_file_name = format!("{web_domain}_{number}.{ext}");
        }

        let mut file = File::create(&default_file_name)?;
        let bytes = file.write(source.as_bytes())?;

        log_stdout!(
            "{} Successfully saved log as {} ({bytes} bytes written)",
            Info.fmt(),
            default_file_name
        );

        Ok(true)
    } else {
        log_stdout!("{} Use --debug option to enable debug mode", Info.fmt());
        Ok(false)
    }
} // fm save_as

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
    response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.parse::<Mime>().ok())
        .map(|mime| mime.essence_str().to_owned())
        .ok_or_else(|| anyhow!("Could not get Content-Type in {:?}", response.headers()))
}

/// Handles HTTP GET request
///
/// # Errors
/// - `Request fails`
/// - `Get response body fails`
#[inline]
pub async fn get_request(
    website_url: &str,
    headers: &[(HeaderName, HeaderValue)],
    debug_enable: bool,
) -> Result<(), anyhow::Error> {
    let progress = ProgressBar::new_spinner();
    let start_time = Instant::now();

    if let Err(error) = ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner} {msg}")
    {
        return Err(anyhow!("{error}"));
    }

    progress.set_message("Processing...");
    progress.enable_steady_tick(Duration::from_millis(100));

    let mut request = CLIENT.get(website_url);
    for (key, value) in headers {
        request = request.header(key, value);
    }

    let response = request.send().await?;

    let content_type = get_content_type(&response)?;
    let response_body = response.text().await?;

    let mut content_types: HashMap<&str, &str> = HashMap::new();
    _ = content_types.insert("application/json", "json");
    _ = content_types.insert("text/html", "html");
    _ = content_types.insert("text/plain", "txt");
    _ = content_types.insert("text/xml", "xml");
    _ = content_types.insert("application/javascript", "js");

    if let Some(ext) = content_types.get(content_type.as_str()) {
        progress.suspend(|| {
            if let Err(error) = set_color_scheme_output(&response_body, ext) {
                return Err(anyhow!("{error}"));
            };
            if let Err(error) = save_as(website_url, ext, &response_body, debug_enable) {
                return Err(anyhow!("{error}"));
            };
            Ok(())
        })?;
        progress.finish_and_clear();
    } else {
        fatal!("Unsupported Content-Type: {}", content_type);
        log_stdout!("\n{}", response_body);
    }

    let end_time = start_time.elapsed();
    log_stdout!("{} Finished in {:.2?}", Success.fmt(), end_time);

    Ok(())
} // fn get_request
