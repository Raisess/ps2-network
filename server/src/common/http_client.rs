use std::io::{BufWriter, Write};

use futures_util::StreamExt;

pub struct HttpClient {
    __reqwest: reqwest::Client,
    host: String,
}

impl HttpClient {
    pub fn new(host: &str) -> Self {
        Self {
            __reqwest: reqwest::Client::new(),
            host: host.to_string(),
        }
    }

    pub async fn download(
        url: &str,
        destination_path: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut file = BufWriter::new(std::fs::File::create(destination_path)?);

        let mut stream = reqwest::get(url).await?.bytes_stream();
        while let Some(bytes) = stream.next().await {
            file.write_all(&bytes?)?;
        }

        file.flush()?;
        Ok(())
    }

    pub async fn get<R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<R, reqwest::Error> {
        let request = self.__reqwest.get(self.url(path)).send().await?;
        let response = request.text().await?;
        Ok(serde_json::from_str(response.as_str()).unwrap())
    }

    pub async fn post<R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &str,
    ) -> Result<R, reqwest::Error> {
        let request = self
            .__reqwest
            .post(self.url(path))
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;
        let response = request.text().await?;
        Ok(serde_json::from_str(response.as_str()).unwrap())
    }

    fn url(&self, path: &str) -> String {
        format!("{}/{path}", self.host)
    }
}
