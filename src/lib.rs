//! Serpscraper Library
//!
//! Provides functionality to fetch search results from Google via SerpApi,
//! and process the resulting links into Markdown content.

use crate::types::*;
use html_to_markdown_rs::convert;
use only_scraper;
use readability_js::Readability;
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;

pub mod types;

pub async fn get_markdown_for_query(
    query: &str,
    api_key: &str,
) -> anyhow::Result<String> {
    let mut params = HashMap::<String, String>::new();
    params.insert("q".to_string(), query.to_string());
    params.insert(
        "location".to_string(),
        "Austin, Texas, United States".to_string(),
    );
    params.insert("hl".to_string(), "en".to_string());
    params.insert("gl".to_string(), "us".to_string());
    params.insert("google_domain".to_string(), "google.com".to_string());

    let search = SerpApiSearch::google(params, api_key.to_string());

    let results = search.json().await.map_err(|e| anyhow::anyhow!(e.to_string()))?;
    let respose: SerpResponse = serde_json::from_value(results.clone())?;

    // We will store our "tickets" (handles) here
    let mut handles = vec![];

    for result in respose.organic_results.iter() {
        let link = result.link.clone();

        let handle = tokio::spawn(async move {
            // Note: only_scraper::scrape might be blocking or heavy async.
            let html = only_scraper::scrape(link).map_err(|e| anyhow::anyhow!(e))?;

            let reader = Readability::new().map_err(|e| anyhow::anyhow!(e))?;
            let article = reader.parse(&html).map_err(|e| anyhow::anyhow!(e))?;
            let md = convert(&article.content, None).map_err(|e| anyhow::anyhow!(e))?;

            Ok::<String, anyhow::Error>(md)
        });

        handles.push(handle);
    }

    // 3. Collect the results
    let mut content = String::new();

    for handle in handles {
        match handle.await {
            Ok(task_result) => {
                match task_result {
                    Ok(md_text) => {
                        content.push_str(&md_text);
                        content.push_str("\n========================\n");
                    }
                    Err(e) => eprintln!("Task failed inner logic: {}", e),
                }
            }
            Err(e) => eprintln!("Task panicked or was cancelled: {}", e),
        }
    }

    Ok(content)
}
