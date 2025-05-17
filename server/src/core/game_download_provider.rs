use serde::Serialize;

pub mod crocdb;

#[derive(Debug, Serialize)]
pub struct GameDownloadData {
    pub name: String,
    pub filename: String,
    pub host: String,
    pub size: u64,
    pub url: String,
}

#[async_trait::async_trait]
pub trait GameDownloadProvider {
    async fn list(&self, search_key: &str) -> Vec<GameDownloadData>;
}
