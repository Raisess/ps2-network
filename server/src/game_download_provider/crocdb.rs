use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::common::http_client::HttpClient;
use super::{GameDownloadData, GameDownloadProvider};

const CROCDB_API_HOST: &str = "https://api.crocdb.net";

pub struct CrocdbDownloadProvider {
    http_client: HttpClient,
}

impl CrocdbDownloadProvider {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(CROCDB_API_HOST),
        }
    }
}

#[async_trait]
impl GameDownloadProvider for CrocdbDownloadProvider {
    async fn list(&self, search_key: &str) -> Vec<GameDownloadData> {
        let payload = r#"
          {
            "search_key": "{search_key}",
            "platforms": ["ps2"],
            "max_results": 5,
            "page": 1
          }
        "#
        .replace("{search_key}", search_key);

        match self.http_client.post::<Response>("/search", &payload).await {
            Err(err) => panic!("TODO: Failed to search {err}"),
            Ok(response) => response
                .data
                .results
                .iter()
                .map(move |item| GameDownloadData {
                    name: item.links[0].name.clone(),
                    filename: item.links[0].filename.clone(),
                    host: item.links[0].host.clone(),
                    size: item.links[0].size.clone(),
                    url: item.links[0].url.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize, Serialize)]
struct Data {
    results: Vec<ResultItem>,
    current_results: u32,
    total_results: u32,
    current_page: u32,
    total_pages: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResultItem {
    slug: String,
    links: Vec<LinkItem>, // Multiple links
}

#[derive(Debug, Deserialize, Serialize)]
struct LinkItem {
    name: String,
    r#type: String, // "type" is a reserved keyword, so we use `r#type` to escape it
    format: String,
    url: String,
    filename: String,
    host: String,
    size: u64,
    size_str: String,
    source_url: String,
}
