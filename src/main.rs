mod types;
use crate::types::*;
use html_to_markdown_rs::convert;
use only_scraper;
use readability_js::Readability;
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SERPER_API_KEY").expect("Missing SERPER_API_KEY env variable");
    dbg!(&api_key);
    let mut params = HashMap::<String, String>::new();
    params.insert("q".to_string(), "best practic to write a python fonction".to_string());
    params.insert("location".to_string(), "Austin, Texas, United States".to_string());
    params.insert("hl".to_string(), "en".to_string());
    params.insert("gl".to_string(), "us".to_string());
    params.insert("google_domain".to_string(), "google.com".to_string());

    let search = SerpApiSearch::google(params, api_key);

    let results = search.json().await?;
    let respose: SerpResponse = serde_json::from_value(results.clone())?;
    let mut content = String::new();
    for (index, result) in respose.organic_results.iter().enumerate() {
        //println!("{}. {} ({})", index + 1, result.title, result.link);
        let html = only_scraper::scrape(result.link.clone())?;
        //let md = rewrite_html(&html, false);
        println!("--------------------");

        //dbg!(&md);
        //let md2 = convert(&html, None)?;
        println!("========================");
        //dbg!(&md2);
        let reader = Readability::new()?;
        let article = reader.parse(&html)?;
        //let md = rewrite_html(&article.content, false);
        let md = convert(&article.content, None)?;
        content.push_str(&md);
    }
    dbg!(&content);
    Ok(())
}
