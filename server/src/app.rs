use axum::extract::Query;

use crate::handler::add_download_to_queue::AddDownloadToQueueHandler;
use crate::handler::list_downloads_on_queue::ListDownloadsOnQueueHandler;
use crate::handler::list_games::ListGamesHandler;
use crate::handler::Handler;

pub struct App {
    port: u16,
}

impl App {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub async fn listen(&self) -> () {
        let router = axum::Router::new()
          .route("/ping", axum::routing::get(ping))
          .route("/search", axum::routing::get(search))
          .route("/downloads", axum::routing::get(list_downloads))
          .route("/download", axum::routing::get(download));

        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, router).await.unwrap();
    }
}

async fn ping<'a>() -> &'a str {
    "pong"
}

#[derive(serde::Deserialize)]
struct SearchQuery {
    key: String,
}

async fn search(query: Query<SearchQuery>) -> String {
    let handler = ListGamesHandler {
        search_key: query.key.clone(),
    };

    let response = handler.handle().await;
    serde_json::to_string(&response).unwrap()
}

async fn list_downloads() -> String {
    let handler = ListDownloadsOnQueueHandler;
    let response = handler.handle().await;
    serde_json::to_string(&response).unwrap()
}

#[derive(serde::Deserialize)]
struct DownloadQuery {
    id: String,
}

async fn download(query: Query<DownloadQuery>) -> () {
    let handler = AddDownloadToQueueHandler {
        game_id: query.id.clone(),
    };

    let _ = handler.handle().await;
}
