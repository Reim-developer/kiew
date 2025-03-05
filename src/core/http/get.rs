use crate::log_stdout;
use crate::ultis::content_type::ContentType;
use crate::{colors::LogLevel::Error, colors::LogLevel::Info, colors::LogLevel::Success, fatal};
use anyhow::{anyhow, Ok, Result};
use chrono::Local;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use mime::Mime;
use owo_colors::OwoColorize;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::{Client, Response};
use std::fs::File;
use std::io::Stdout;
use std::path::Path;
use std::time::Duration;
use std::{
    io::{self, Write},
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

    /// Stdout
    static ref stdout: Stdout = io::stdout();
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
    stdout.lock().write_all(&buffer)?;

    Ok(())
} // fn set_color_scheme_output

/// Get content type with response
fn get_content_type(response: &Response) -> Result<ContentType, anyhow::Error> {
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .and_then(|str| str.parse::<Mime>().ok())
        .map(|mime| mime.essence_str().to_owned())
        .ok_or_else(|| anyhow!("Could not get Content-Type in {:?}", response.headers()))?;

    Ok(ContentType::get(&content_type))
}

/// Handles HTTP GET request
///
/// # Errors
/// - `Request fails`
/// - `Get response body fails`
#[inline]
async fn get_request(
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

    if let ContentType::Other(ref unknown_type) = content_type {
        progress.finish_and_clear();
        fatal!("{} Unknown Content-Type: {}", Error.fmt(), unknown_type);
        log_stdout!("\n{}", response_body);
    } else {
        progress.suspend(|| {
            let ext = content_type.get_extension();
            if let Err(error) = set_color_scheme_output(&response_body, ext) {
                return Err(anyhow!("{error}"));
            };
            if let Err(error) = save_as(website_url, ext, &response_body, debug_enable) {
                return Err(anyhow!("{error}"));
            };
            Ok(())
        })?;
        progress.finish_and_clear();
    }

    let end_time = start_time.elapsed();
    log_stdout!("{} Finished in {:.2?}", Success.fmt(), end_time);

    Ok(())
} // fn get_request

/// Get details response of website
/// instead of response body
async fn get_response_details(website_url: &str, debug: bool) -> Result<(), anyhow::Error> {
    let start_time = Instant::now();
    let response = CLIENT.get(website_url).send().await?;
    let content_type = response
        .headers()
        .get("Content-Type")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("Unknown Content Type");

    let content_length = response
        .content_length()
        .map_or("Unknown content length".to_owned(), |ct| ct.to_string());
    let status_code = response.status();
    let http_version = response.version();
    let headers = response.headers();
    let url_parse = Url::parse(website_url)?;
    let host_name = url_parse.host_str().unwrap_or("Unknown hostname");

    let time_now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut details = vec![format!(
        "
➤ Basic Information:
\t Website {website_url}
\t Hostname: {host_name}
\t Status Code: {status_code}
\t Content Type: {content_type}
\t Content length: {content_length}
\t HTTP Version: {http_version:?}
\t Date: {time_now}
        "
    )];

    let response_headers: Vec<String> = headers
        .iter()
        .map(|(key, value)| {
            format!(
                "\t {}: {}",
                key.as_str().to_uppercase(),
                value.to_str().unwrap_or("Unkown")
            )
        })
        .collect();

    details.push("➤ Headers:".to_owned());
    details.push(response_headers.join("\n"));

    if debug {
        let mut file_name = format!("{host_name}.txt");

        let mut number: i32 = 0;
        while Path::new(&file_name).exists() {
            number = number.saturating_add(1);
            file_name = format!("{host_name}_{number}.txt");
        }

        let mut file = File::create(&file_name)?;
        let bytes = file.write(details.join("\n").as_bytes())?;
        
        writeln!(&*stdout, "{}", details.join("\n"))?;
        log_stdout!(
            "{} Saved as {file_name} ({bytes} bytes), {:.2?}",
            Success.fmt(),
            start_time.elapsed()
        );
    } else {
        writeln!(&*stdout, "{}", details.join("\n"))?;
        log_stdout!(
            "{} Finished in {:.2?}. Use --debug to enable debug mode",
            Success.fmt(),
            start_time.elapsed()
        );
    }

    Ok(())
}

/// Handles CLI options:
///
/// # Errors
/// - `get_request fn` fails
#[inline]
pub async fn match_options_get(
    website_url: &str,
    headers: &[(HeaderName, HeaderValue)],
    debug_enable: bool,
    details: bool,
) -> Result<(), anyhow::Error> {
    if details {
        get_response_details(website_url, debug_enable).await?;
    } else {
        get_request(website_url, headers, debug_enable).await?;
        log_stdout!(
            "{} Use 'kiew -I -w {website_url}' to show request details instead of response body",
            Info.fmt()
        );
    }
    Ok(())
}
