use std::collections::VecDeque;

use crate::core::game_download_provider::GameDownloadData;
use crate::core::queue::queue;

use super::Handler;

pub struct ListDownloadsHandler;

#[async_trait::async_trait]
impl Handler<VecDeque<GameDownloadData>> for ListDownloadsHandler {
    async fn handle(&self) -> VecDeque<GameDownloadData> {
        queue().lock().unwrap().clone()
    }
}
