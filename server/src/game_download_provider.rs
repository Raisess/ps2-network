pub mod crocdb;

use async_trait::async_trait;

#[derive(Debug)]
pub struct GameDownloadData {
    pub name: String,
    pub filename: String,
    pub host: String,
    pub size: u64,
    pub url: String,
}

#[async_trait]
pub trait GameDownloadProvider {
    async fn list(&self, search_key: &str) -> Vec<GameDownloadData>;
}
