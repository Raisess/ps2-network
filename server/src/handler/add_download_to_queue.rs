use crate::core::download_provider::crocdb::CrocdbDownloadProvider;
use crate::core::download_provider::{DownloadProvider, DownloadStatus};
use crate::core::queue::queue;

use super::Handler;

pub struct AddDownloadToQueueHandler {
    pub game_id: String,
}

#[async_trait::async_trait]
impl Handler<()> for AddDownloadToQueueHandler {
    async fn handle(&self) -> () {
        let mut queue = queue().lock().await;
        for item in queue.iter() {
            if item.id == self.game_id {
                return ();
            }
        }

        let provider = CrocdbDownloadProvider::new();
        let mut game_download_data = provider.get(&self.game_id).await;
        game_download_data.status = Some(DownloadStatus::PENDING);
        queue.push_back(game_download_data);
    }
}
