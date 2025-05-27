use crate::core::download_provider::crocdb::CrocdbDownloadProvider;
use crate::core::download_provider::{DownloadData, DownloadProvider};

use super::Handler;

// @TODO: implement pagination
pub struct ListGamesHandler {
    pub search_key: String,
}

#[async_trait::async_trait]
impl Handler<Vec<DownloadData>> for ListGamesHandler {
    async fn handle(&self) -> Vec<DownloadData> {
        let provider = CrocdbDownloadProvider::new();
        provider.list(&self.search_key, None, None).await
    }
}
