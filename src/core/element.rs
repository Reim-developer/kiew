use anyhow::Result;
use reqwest::Client;
use scraper::Html;

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

    let response = client.get(website).header(reqwest::header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
    .send().await?;

    let html_content = response.text().await?;

    let document = Html::parse_document(&html_content);

    let selector_query = scraper::Selector::parse(element)
        .map_err(|error| anyhow::anyhow!("Cannot parse element with error {}", error))?;

    let element_count = document.select(&selector_query).count();

    match element_count {
        0 => println!("Could not find any element."),
        1 => println!("Find one element: {}.", element_count),
        _ => println!("Found many elements: {}.", element_count),
    }

    Ok(())
} // fn element_count
