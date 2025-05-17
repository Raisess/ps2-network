use crate::core::game_download_provider::crocdb::CrocdbDownloadProvider;
use crate::core::game_download_provider::{GameDownloadProvider, GameDownloadStatus};
use crate::core::queue::queue;

use super::Handler;

pub struct DownloadHandler {
    pub game_id: String,
}

#[async_trait::async_trait]
impl Handler<()> for DownloadHandler {
    async fn handle(&self) -> () {
        let clone_queue = queue().lock().unwrap().clone();
        for item in clone_queue.iter() {
            if item.id == self.game_id {
                return ();
            }
        }

        let provider = CrocdbDownloadProvider::new();
        let mut game_download_data = provider.get(&self.game_id).await;
        game_download_data.status = Some(GameDownloadStatus::PENDING);
        queue().lock().unwrap().push_back(game_download_data);
    }
}
