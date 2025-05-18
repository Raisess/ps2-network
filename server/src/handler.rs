pub mod add_download_to_queue;
pub mod list_downloads_on_queue;
pub mod list_games;
pub mod process_download_on_queue;

#[async_trait::async_trait]
pub trait Handler<T> {
    async fn handle(&self) -> T;
}
