use std::{
    fs::File, io::Write, time::{Duration, Instant}
};

use crate::{colors::{self, LogLevel}, errors::ErrorsType};
use serde_json::{json, to_string_pretty, Map, Value};
use anyhow::{anyhow, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use prettytable::{format, Cell, Row, Table};
use reqwest::{header::USER_AGENT, Client, Url};
use scraper::{Html, Selector};

/// For save log as txt file
fn save_as_txt(website: &str, website_element: &str, success_color: &String, info_color: &str, table: &Table) -> Result<()> {
    let website_url = Url::parse(website).map_err(|error| anyhow!("{error}"))?;

    let website_name = website_url
        .host_str()
        .ok_or_else(|| anyhow!(ErrorsType::UrlNotFound.as_str()))?;

    let mut log_file =
        File::create(format!("{website_name}.txt")).map_err(|error| anyhow!("{error}"))?;

    table
        .print(&mut log_file)
        .map_err(|error| anyhow!("{error}"))?;

    println!("{success_color} Saved log as {website_name}.txt");
    println!("{info_color} To customize your save log, just use kiew find -w {website} -e {website_element} --debug -l json/markdown/html");

    Ok(())
}

/// For save log as JSON
fn save_json(website: &str, elements: Vec<(&str, &str, &str)>, success_color: &String) -> Result<()> {
    let website_url = Url::parse(website).map_err(|error| 
        anyhow!("{error}")
    )?;
    let website_name = website_url.host_str().ok_or_else(|| anyhow!(ErrorsType::UrlNotFound.as_str()))?;

    let mut element_map: Map<String, Value> = Map::new();
    for (element_type, element_id, element_class) in elements {
        let entry = element_map.entry(element_type.to_string()).or_insert_with(|| json!([]));

        if let Value::Array(array) = entry {
            array.push(json!({
                "Element ID": element_id,
                "Element Class": element_class
            }));
        };
    }

    let json_raw_data = json!({
        "Element": element_map
    });

    let json_str= to_string_pretty(&json_raw_data).map_err(|error| {
        anyhow!("{error}")
    })?;

    let mut log_file = File::create(format!("{website_name}.json")).map_err(|error| {
        anyhow!("{error}")
    })?;
    log_file.write_all(json_str.as_bytes()).map_err(|error| {
        anyhow!("{error}")
    })?;

    println!("{success_color} Saved log as {website_name}.json");

    Ok(())
}

/// Find HTML element of website and print this info, if exist.
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
/// # Errors
///
/// - `REQUEST_FAILED` Request is fails.
/// - `HTML_PARSE_FAILED` Parsing HTML is fails.
/// - `ELEMENT_NOT_FOUND` Could not find any element.
///
pub async fn find_element(website: &str, element: &str, debug_mode: bool, log_type: &str) -> Result<()> {
    let client = Client::new();
    let success_color = colors::LogLevel::Success.fmt();
    let process_bar = ProgressBar::new_spinner();
    let mut element_vec = Vec::new();
    let mut table = Table::new();

    let start_time = Instant::now();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(Row::new(vec![
        Cell::new("Element Type"),
        Cell::new("Element ID"),
        Cell::new("Element Class"),
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
    .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .send().await.with_context(|| ErrorsType::RequestFailed.as_str())?;

    let html_content = response
        .text()
        .await
        .with_context(|| ErrorsType::HtmlParseFailed.as_str())?;

    let website_source = Html::parse_document(&html_content);

    let selector_query = Selector::parse(element).map_err(|error| anyhow!("{}", error))?;

    let mut elements = website_source.select(&selector_query).peekable();

    if elements.peek().is_none() {
        return Err(anyhow!("{}", ErrorsType::ElementNotFound.as_str()));
    }

    for webpage_element in elements {
        let element_type = webpage_element.value().name();
        let element_class = webpage_element.value().attr("class").unwrap_or("No class");
        let element_id = webpage_element.value().attr("id").unwrap_or("No id");

        table.add_row(Row::new(vec![
            Cell::new(element_type),
            Cell::new(element_id),
            Cell::new(element_class),
        ]));

        element_vec.push((element_type, element_id, element_class));
    }

    process_bar.suspend(|| {
        table.printstd();
    });
    let end_time = start_time.elapsed();

    process_bar.finish_and_clear();
    println!("{success_color} Finished in {end_time:.2?}");

    if debug_mode {
        let info_color = LogLevel::Info.fmt();
        match log_type.to_lowercase().as_str() {
            "json" => save_json(website, element_vec, &success_color)?,
            "md" | "markdown" => println!("Markdown debug"),
            _=> save_as_txt(website, element, &success_color, &info_color, &table)?,
        }
    } else {
        println!("{success_color} To enable debug mode, just type: kiew find -w {website} -e {element} --debug");
    }

    Ok(())
} // fn find_element
