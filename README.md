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

## Dependencies

*   `anyhow`: Error handling.
*   `html-to-markdown-rs`: HTML to Markdown conversion.
*   `only_scraper`: Web scraping.
*   `readability-js`: Article extraction.
*   `serde` & `serde_json`: JSON serialization/deserialization.
*   `serpapi-search-rust`: Google Search API client.
*   `tokio`: Async runtime.
