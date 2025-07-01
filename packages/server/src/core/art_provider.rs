pub mod internet_archive;

#[derive(Debug)]
pub struct ArtData {
    pub bg_file: String,
    pub bg_url: String,
    pub cov_file: String,
    pub cov_url: String,
    pub lgo_file: String,
    pub lgo_url: String,
}

#[async_trait::async_trait]
pub trait ArtProvider {
    // @TODO: improve error handling to not panic
    async fn get(&self, serial: &str) -> ArtData;
}
