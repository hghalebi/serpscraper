//! Serpscraper Binary
//!
//! CLI entry point that accepts a query string, fetches results via `serpscraper` lib,
//! and prints the aggregated Markdown to stdout.

use serpscraper::get_markdown_for_query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("SERPER_API_KEY").expect("Missing SERPER_API_KEY env variable");

    // Get query from command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <query>", args.get(0).unwrap_or(&"serpscraper".to_string()));
        std::process::exit(1);
    }
    let query = args[1..].join(" ");

    let markdown_content = get_markdown_for_query(&query, &api_key).await?;
    println!("{}", markdown_content);

    Ok(())
}