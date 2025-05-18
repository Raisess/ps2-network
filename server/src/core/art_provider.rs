pub mod internet_archive;

#[derive(Debug)]
pub struct ArtData {
    pub bg_url: String,
    pub cov_url: String,
    pub logo_url: String,
}

#[async_trait::async_trait]
pub trait ArtProvider {
    // @TODO: improve error handling to not panic
    async fn get(&self, serial: &str) -> ArtData;
}
