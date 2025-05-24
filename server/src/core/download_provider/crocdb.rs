use serde::{Deserialize, Serialize};

use super::{DownloadData, DownloadProvider};
use crate::common::http_client::HttpClient;

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

#[async_trait::async_trait]
impl DownloadProvider for CrocdbDownloadProvider {
    // @TODO: improve error handling to not panic
    async fn list(&self, search_key: &str) -> Vec<DownloadData> {
        let payload = r#"
          {
            "search_key": "{search_key}",
            "platforms": ["ps2"],
            "max_results": 5,
            "page": 1
          }
        "#
        .replace("{search_key}", search_key);

        match self
            .http_client
            .post::<ListResponse>("search", &payload)
            .await
        {
            Err(err) => panic!("TODO: Failed to get {err}"),
            Ok(response) => response
                .data
                .results
                .iter()
                .map(move |item| DownloadData {
                    id: item.slug.clone(),
                    name: item.links[0].name.clone(),
                    filename: item.links[0].filename.clone(),
                    host: item.links[0].host.clone(),
                    size: item.links[0].size.clone(),
                    url: item.links[0].url.clone(),
                    serial: None,
                    status: None,
                })
                .collect(),
        }
    }

    // @TODO: improve error handling to not panic
    async fn get(&self, id: &str) -> DownloadData {
        let payload = r#"{ "slug": "{slug}" }"#.replace("{slug}", id);

        match self
            .http_client
            .post::<GetResponse>("entry", &payload)
            .await
        {
            Err(err) => panic!("TODO: Failed to get {err}"),
            Ok(response) => DownloadData {
                id: response.data.entry.slug.clone(),
                name: response.data.entry.links[0].name.clone(),
                filename: response.data.entry.links[0].filename.clone(),
                host: response.data.entry.links[0].host.clone(),
                size: response.data.entry.links[0].size.clone(),
                url: response.data.entry.links[0].url.clone(),
                serial: None,
                status: None,
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct ListResponse {
    data: ListData,
}

#[derive(Debug, Deserialize, Serialize)]
struct ListData {
    results: Vec<ResultItem>,
    current_results: u32,
    total_results: u32,
    current_page: u32,
    total_pages: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetResponse {
    data: GetData,
}

#[derive(Debug, Deserialize, Serialize)]
struct GetData {
    entry: ResultItem,
}

#[derive(Debug, Deserialize, Serialize)]
struct ResultItem {
    slug: String,
    links: Vec<LinkItem>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LinkItem {
    name: String,
    r#type: String,
    format: String,
    url: String,
    filename: String,
    host: String,
    size: u64,
    size_str: String,
    source_url: String,
}
