use std::time::{Duration, Instant};

use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format::consts::FORMAT_BOX_CHARS, Cell, Row, Table};
use reqwest::Client;
use scraper::Html;

use crate::{colors::LogLevel, errors::ErrorsType};

/// Counter the number of elements matching a CSS query on a given website.
///
/// # Example
/// ```rust
/// let my_website = "https://example.com";
/// let my_css_query = "div#example"; // div tag with example id
///
/// element_count(my_website, my_css_query);
///
/// ```
/// # Parameters:
/// - `website` Website you want to check.
/// - `element` Your CSS query. Use * to query all elements.
///
pub async fn element_count(website: &str, element: &str) -> Result<()> {
    let client = Client::new();
    let success_color = LogLevel::Success.fmt();
    let info_color = LogLevel::Info.fmt();
    let process_bar = ProgressBar::new_spinner();
    let start_time = Instant::now();

    let mut table = Table::new();
    table.set_format(*FORMAT_BOX_CHARS);
    table.add_row(Row::new(vec![
        Cell::new("Element Type"),
        Cell::new("Element(s) Count"),
    ]));

    if let Err(error) = ProgressStyle::default_spinner()
        .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
        .template("{spinner} {msg}")
    {
        return Err(anyhow!("{}", error));
    }

    process_bar.set_message("Processing...");
    process_bar.enable_steady_tick(Duration::from_millis(100));

    let response = client.get(website)
        .timeout(Duration::from_secs(15))
        .header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .send().await.with_context(|| ErrorsType::RequestFailed.as_str())?;

    let html_content = response
        .text()
        .await
        .with_context(|| ErrorsType::HtmlParseFailed.as_str())?;

    let document = Html::parse_document(&html_content);

    let selector_query =
        scraper::Selector::parse(element).map_err(|error| anyhow::anyhow!("{}", error))?;

    let element_count = document.select(&selector_query).count();

    if element_count == 0 {
        return Err(anyhow!("{}", ErrorsType::ElementNotFound.as_str()));
    }

    let end_time = start_time.elapsed();
    process_bar.finish_and_clear();

    table.add_row(Row::new(vec![
        Cell::new(element),
        Cell::new(format!("{element_count}").as_str()),
    ]));
    table.printstd();

    println!("{success_color} Finished in {end_time:.2?}");
    println!(
        "{info_color} To find any specify element infomation, just use: kiew find -w {website} -e {element}"
    );
    Ok(())
} // fn element_count
