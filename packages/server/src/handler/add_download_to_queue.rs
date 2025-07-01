use crate::core::database;
use crate::core::download_provider::crocdb::CrocdbDownloadProvider;
use crate::core::download_provider::{DownloadProvider, DownloadStatus};

use super::Handler;

pub struct AddDownloadToQueueHandler {
    pub game_id: String,
}

#[async_trait::async_trait]
impl Handler<()> for AddDownloadToQueueHandler {
    async fn handle(&self) -> () {
        let game_id = self.game_id.as_str();

        if !database::exists(&game_id).await {
            let provider = CrocdbDownloadProvider::new();
            let mut game_download_data = provider.get(game_id).await;
            game_download_data.status = Some(DownloadStatus::PENDING);
            database::insert(&game_id, &game_download_data).await;
        }
    }
}
