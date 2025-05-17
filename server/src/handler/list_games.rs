use crate::core::game_download_provider::crocdb::CrocdbDownloadProvider;
use crate::core::game_download_provider::{GameDownloadData, GameDownloadProvider};

use super::Handler;

pub struct ListGamesHandler {
    pub search_key: String,
}

#[async_trait::async_trait]
impl Handler<Vec<GameDownloadData>> for ListGamesHandler {
    async fn handle(&self) -> Vec<GameDownloadData> {
        let provider = CrocdbDownloadProvider::new();
        provider.list(&self.search_key).await
    }
}
