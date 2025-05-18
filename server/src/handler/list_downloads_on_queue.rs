use std::collections::VecDeque;

use crate::core::download_provider::DownloadData;
use crate::core::queue::queue;

use super::Handler;

pub struct ListDownloadsOnQueueHandler;

#[async_trait::async_trait]
impl Handler<VecDeque<DownloadData>> for ListDownloadsOnQueueHandler {
    async fn handle(&self) -> VecDeque<DownloadData> {
        queue().lock().await.clone()
    }
}
