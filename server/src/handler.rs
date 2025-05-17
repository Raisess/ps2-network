pub mod list_games;

#[async_trait::async_trait]
pub trait Handler<T> {
    async fn handle(&self) -> T;
}
