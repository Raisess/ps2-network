use crate::core::database;
use crate::core::download_provider::DownloadData;

use super::Handler;

pub struct ListDownloadsOnQueueHandler;

#[async_trait::async_trait]
impl Handler<Vec<DownloadData>> for ListDownloadsOnQueueHandler {
    async fn handle(&self) -> Vec<DownloadData> {
        database::list().await
    }
}
