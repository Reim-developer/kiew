use crate::colors::LogLevel::{Info, Success};
use crate::errors::ErrorsType::ElementNotFound;
use anyhow::{anyhow, Result};
use prettytable::{format::consts::FORMAT_BOX_CHARS, Cell, Row, Table};
use reqwest::{header::USER_AGENT, Client};
use scraper::{Html, Selector};
use std::borrow::Cow;
use std::time::{Duration, Instant};

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

/// Crawl all href elments in website
/// # Argument:
/// - `website_url:` Websie you want crawl.
///
/// # Errors
/// `REQUEST_FAILED` Get response/Request is fails
///
pub async fn href_scraper(website_url: &str) -> Result<(), anyhow::Error> {
    let client = Client::builder().timeout(Duration::from_secs(15)).build()?;

    let start_time = Instant::now();

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

    if href_element.is_empty() {
        return Err(anyhow!("{}", ElementNotFound.as_str()));
    }

    let mut table = Table::new();
    table.set_format(*FORMAT_BOX_CHARS);
    table.add_row(Row::new(vec![
        Cell::new("Number"),
        Cell::new("Element Type"),
        Cell::new("Href URL"),
    ]));

    let mut index: i32 = 0;
    for href in &href_element {
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

    let end_time = start_time.elapsed();
    println!("{} Finished in: {end_time:.2?}\n{} Use kiew crawl href --web {website_url} --debug to enable debug mode",  Success.fmt(), Info.fmt());
    Ok(())
}
