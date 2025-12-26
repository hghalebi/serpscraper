# Serpscraper

A CLI tool to fetch and convert search results into Markdown.

## Requirements

*   **Rust**: Stable toolchain (edition 2024).
*   **Environment Variables**:
    *   `SERPER_API_KEY`: API key for [SerpApi](https://serpapi.com/).

## Usage

```bash
# Set your API key
export SERPER_API_KEY="your_secret_key"

# Run the search
serpscraper "how to write better rust code"
```

## Library Usage

Add `serpscraper` to your `Cargo.toml`:

```toml
[dependencies]
serpscraper = "0.1"
tokio = { version = "1", features = ["full"] }
```

Use it in your Rust code:

```rust
use serpscraper::get_markdown_for_query;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let api_key = std::env::var("SERPER_API_KEY")?;
    let query = "rust async tutorial";

    let markdown = get_markdown_for_query(query, &api_key).await?;
    println!("{}", markdown);

    Ok(())
}
```

## Dependencies

*   `anyhow`: Error handling.
*   `html-to-markdown-rs`: HTML to Markdown conversion.
*   `only_scraper`: Web scraping.
*   `readability-js`: Article extraction.
*   `serde` & `serde_json`: JSON serialization/deserialization.
*   `serpapi-search-rust`: Google Search API client.
*   `tokio`: Async runtime.
