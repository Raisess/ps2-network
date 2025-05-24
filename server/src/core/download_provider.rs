use serde::{Deserialize, Serialize};

pub mod crocdb;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum DownloadStatus {
    PENDING,
    DOWNLOADING,
    EXTRACTING,
    INSTALLING,
    DOWNLOADINGART,
    DONE,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DownloadData {
    pub id: String,
    pub name: String,
    pub filename: String,
    pub host: String,
    pub size: u64,
    pub url: String,
    pub serial: Option<String>,
    pub status: Option<DownloadStatus>,
}

impl DownloadData {
    pub fn step(&mut self, status: DownloadStatus) -> () {
        self.status = Some(status);
    }
}

#[async_trait::async_trait]
pub trait DownloadProvider {
    // @TODO: improve error handling to not panic
    async fn list(&self, search_key: &str) -> Vec<DownloadData>;
    // @TODO: improve error handling to not panic
    async fn get(&self, id: &str) -> DownloadData;
}
