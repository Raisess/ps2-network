use super::{ArtData, ArtProvider};

const INTERNET_ARCHIVE_HOST: &str = "https://ia600701.us.archive.org/view_archive.php";

pub struct InternetArchiveArtProvider;

impl InternetArchiveArtProvider {
    fn format_url(&self, serial: &str, pattern: &str) -> String {
        format!("{INTERNET_ARCHIVE_HOST}?archive=/11/items/OPLM_ART_2024_09/OPLM_ART_2024_09.zip&file=PS2%2F{serial}%2F{serial}_{pattern}.png")
    }

    fn format_file(&self, serial: &str, pattern: &str) -> String {
        format!("{serial}_{pattern}.png")
    }
}

#[async_trait::async_trait]
impl ArtProvider for InternetArchiveArtProvider {
    async fn get(&self, serial: &str) -> ArtData {
        ArtData {
            bg_file: self.format_file(serial, "BG"),
            bg_url: self.format_url(serial, "BG_00"),
            cov_file: self.format_file(serial, "COV"),
            cov_url: self.format_url(serial, "COV"),
            lgo_file: self.format_file(serial, "LGO"),
            lgo_url: self.format_url(serial, "LGO"),
        }
    }
}
