use server::app::App;
use server::common::config::Config;
use server::core::database;
use server::core::download_provider::{DownloadData, DownloadStatus};
use server::handler::process_download_on_queue::ProcessDownloadOnQueueHandler;
use server::handler::Handler;

#[tokio::main]
async fn main() -> () {
    assert_ne!(Config::source_path(), "");
    assert_ne!(Config::target_path(), "");

    tracing_subscriber::fmt::init();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<DownloadData>(100);
    let previous_added_downloads = database::list().await;
    for previous_added_download in previous_added_downloads {
        tx.send(previous_added_download).await.unwrap();
    }

    tokio::spawn(async move {
        loop {
            match database::first().await {
                Some(game_download_data) => {
                    let is_peding = game_download_data
                        .status
                        .as_ref()
                        .is_some_and(|i| *i == DownloadStatus::PENDING);
                    if is_peding {
                        tx.send(game_download_data).await.unwrap();
                    }
                }
                None => {}
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    tokio::spawn(async move {
        match rx.recv().await {
            Some(download_data) => {
                let handler = ProcessDownloadOnQueueHandler {
                    download_data: download_data.clone(),
                };
                handler.handle().await;
            }
            None => {}
        }
    });

    let app = App::new(8080);
    app.listen().await
}
