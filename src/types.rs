use serde::Deserialize;

// 1. The Root Structure representing the entire JSON object
#[derive(Deserialize, Debug)]
pub struct SerpResponse {
    pub search_metadata: SearchMetadata,
    pub search_information: SearchInformation,
    pub organic_results: Vec<OrganicResult>,
    pub pagination: Pagination,
}

// 2. The core data: Organic Results
#[derive(Deserialize, Debug)]
pub struct OrganicResult {
    pub title: String,
    pub link: String,
    pub position: u32, // Unsigned integer (positive only)
    pub snippet: Option<String>, // Option handles fields that might be null or missing
    pub displayed_link: Option<String>,
    pub source: Option<String>,
    pub thumbnail: Option<String>,

    // Arrays in JSON become Vectors (Vec) in Rust
    pub snippet_highlighted_words: Option<Vec<String>>,
}

// 3. Supporting Structures
#[derive(Deserialize, Debug)]
pub struct SearchMetadata {
    pub id: String,
    pub status: String,
    pub created_at: String,
    pub total_time_taken: f64,
}

#[derive(Deserialize, Debug)]
pub struct SearchInformation {
    pub total_results: u64,
    pub time_taken_displayed: f64,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub current: u32,
    pub next: Option<String>,
}