pub mod add_download_to_queue;
pub mod list_downloads_on_queue;
pub mod list_games;

#[async_trait::async_trait]
pub trait Handler<T> {
    async fn handle(&self) -> T;
}
