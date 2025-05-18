use serde::Serialize;

pub mod crocdb;

#[derive(Debug, Clone, Serialize)]
pub enum DownloadStatus {
    PENDING,
    DOWNLOADING,
    EXTRACTING,
    INSTALLING,
    COMPLETED,
}

#[derive(Debug, Clone, Serialize)]
pub struct DownloadData {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub host: String,
    pub size: u64,
    pub url: String,
    pub status: Option<DownloadStatus>,
}

#[async_trait::async_trait]
pub trait DownloadProvider {
    // @TODO: improve error handling to not panic
    async fn list(&self, search_key: &str) -> Vec<DownloadData>;
    // @TODO: improve error handling to not panic
    async fn get(&self, id: &str) -> DownloadData;
}
