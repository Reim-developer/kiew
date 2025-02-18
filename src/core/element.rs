use anyhow::{Context, Result};
use reqwest::Client;
use scraper::Html;

use crate::styles::colors::LogLevel;

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
    let error_color = LogLevel::Error.fmt();
    let success_color = LogLevel::Success.fmt();
    let warn_color = LogLevel::Warning.fmt();

    let response = client.get(website).header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .send().await.with_context(|| format!("{} Cannot get response", error_color))?;

    let html_content = response
        .text()
        .await
        .with_context(|| format!("{} Cannot get HTML source", error_color))?;

    let document = Html::parse_document(&html_content);

    let selector_query = scraper::Selector::parse(element).map_err(|error| {
        anyhow::anyhow!("{} Cannot parse element with error {}", error_color, error)
    })?;

    let element_count = document.select(&selector_query).count();

    match element_count {
        0 => println!("{} Could not find any element.", warn_color),
        1 => println!("{} Found one {} element: {}.", success_color, element, element_count),
        _ => println!("{} Found many {} elements: {}.", success_color, element, element_count),
    }

    Ok(())
} // fn element_count
