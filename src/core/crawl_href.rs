use crate::colors::LogLevel::{Info, Success};
use crate::errors::ErrorsType::ElementNotFound;
use crate::log_stdout;
use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format::consts::FORMAT_BOX_CHARS, Cell, Row, Table};
use reqwest::Url;
use reqwest::{header::USER_AGENT, Client};
use scraper::{ElementRef, Html, Selector};
use std::borrow::Cow;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

/// Save log as txt file format
///
/// # Errors
/// - `parse url fails`
/// - `get host_str fails`
/// - `create file fails`
/// - `write all buffer fails`
///
/// # Return
/// - `Ok` if not error
fn save_as_text(website_url: &str, log_stdout: &[Cow<str>]) -> Result<(), anyhow::Error> {
    let web = Url::parse(website_url)?;
    let host_name = web.host_str().context("Could not get hostname")?;

    let mut log_file = File::create(format!("{host_name}.txt"))?;

    writeln!(log_file, "{}", log_stdout.join("\n"))?;

    log_stdout!("{} Log saved as: {host_name}.txt", Info.fmt());
    Ok(())
}

/// Split URL if this too long
fn split_url(web_url: &str) -> Cow<str> {
    if web_url.chars().count() > 50 {
        let split_pos = web_url
            .char_indices()
            .nth(50)
            .map_or(web_url.len(), |(idx, _)| idx);

        Cow::Owned(format!(
            "{}...",
            web_url.get(..split_pos).unwrap_or(web_url)
        ))
    } else {
        Cow::Borrowed(web_url)
    }
}

/// Small helper function
/// for otpion scraper with out debug.
///
/// This looks like:
///
/// ```bash
/// kiew crawl href --web https://google.com
/// ```
fn scraper_without_debug(mut table: Table, element: Vec<ElementRef<'_>>) {
    let mut index: i32 = 0;
    for href in element {
        let element_type = href.value().name();

        if let Some(href_url) = href.value().attr("href") {
            index = index.saturating_add(1);
            table.add_row(Row::new(vec![
                Cell::new(&index.to_string()),
                Cell::new(element_type),
                Cell::new(&split_url(href_url)),
            ]));
        };
    }
    table.printstd();
}

/// Crawl all href elments in website
/// # Argument:
/// - `website_url:` Websie you want crawl.
///
/// # Errors
/// `REQUEST_FAILED` Get response/Request is fails
///
pub async fn href_scraper(website_url: &str, debug: &str) -> Result<(), anyhow::Error> {
    let client = Client::builder().timeout(Duration::from_secs(15)).build()?;
    let start_time = Instant::now();
    let progress_bar = ProgressBar::new_spinner();

    let response_body = client.get(website_url)
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .send()
    .await?
    .text()
    .await?;

    let web_source = Html::parse_document(&response_body);

    let css_query =
        Selector::parse("a, img, script, link, iframe").map_err(|error| anyhow!("{error}"))?;

    let href_element: Vec<_> = web_source.select(&css_query).collect();

    if let Err(error) = ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner} {msg}")
    {
        return Err(anyhow!("{error}"));
    }

    progress_bar.set_message("Processing...");
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    if href_element.is_empty() {
        return Err(anyhow!("{}", ElementNotFound.as_str()));
    }

    match debug {
        "none" => {
            let mut table = Table::new();
            table.set_format(*FORMAT_BOX_CHARS);
            table.add_row(Row::new(vec![
                Cell::new("Number"),
                Cell::new("Element Type"),
                Cell::new("Href URL"),
            ]));
            progress_bar.suspend(|| {
                scraper_without_debug(table, href_element);
            });

            let end_time = start_time.elapsed();

            progress_bar.finish_and_clear();

            log_stdout!("{} Finished in: {end_time:.2?}", Success.fmt());
            log_stdout!(
                "{} Use kiew crawl href --web {website_url} --debug to enable debug mode",
                Info.fmt()
            );
        }
        "txt" => {
            let mut logs: Vec<Cow<str>> = Vec::new();

            let mut number: i32 = 0;
            for href in href_element {
                let element = href.value().name();

                if let Some(href_url) = href.value().attr("href") {
                    number = number.saturating_add(1);
                    logs.push(Cow::Owned(format!(
                        "Number: {number}\nType: {element}\nURL: {href_url}\n"
                    )));
                }
            }

            if logs.is_empty() {
                return Err(anyhow!("Href not found"));
            }

            progress_bar.suspend(|| {
                if let Err(error) = save_as_text(website_url, &logs) {
                    return Err(anyhow!("{error}"));
                };
                Ok(())
            })?;
            let end_time = start_time.elapsed();
            progress_bar.finish_and_clear();

            log_stdout!("{} Finished in {end_time:.2?}", Success.fmt());
        }
        _ => {
            return Err(anyhow!("File format {debug} are not supported"));
        }
    }

    Ok(())
}
