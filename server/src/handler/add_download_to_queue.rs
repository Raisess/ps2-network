use crate::core::download_provider::crocdb::CrocdbDownloadProvider;
use crate::core::download_provider::DownloadProvider;
use crate::core::queue::queue;

use super::Handler;

pub struct AddDownloadToQueueHandler {
    pub game_id: String,
}

#[async_trait::async_trait]
impl Handler<()> for AddDownloadToQueueHandler {
    async fn handle(&self) -> () {
        let clone_queue = queue().lock().unwrap().clone();
        for item in clone_queue.iter() {
            if item.id == self.game_id {
                return ();
            }
        }

        let provider = CrocdbDownloadProvider::new();
        let game_download_data = provider.get(&self.game_id).await;
        queue().lock().unwrap().push_back(game_download_data);
    }
}
