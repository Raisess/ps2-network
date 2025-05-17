use serde::Serialize;

pub mod crocdb;

#[derive(Debug, Clone, Serialize)]
pub enum GameDownloadStatus {
    PENDING,
    PROCESSING,
    COMPLETED,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameDownloadData {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub host: String,
    pub size: u64,
    pub url: String,
    pub status: Option<GameDownloadStatus>,
}

#[async_trait::async_trait]
pub trait GameDownloadProvider {
    // @TODO: improve error handling to not panic
    async fn list(&self, search_key: &str) -> Vec<GameDownloadData>;
    // @TODO: improve error handling to not panic
    async fn get(&self, id: &str) -> GameDownloadData;
}
