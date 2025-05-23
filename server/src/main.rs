use server::app::App;
use server::common::config::Config;
use server::core::database;
use server::core::download_provider::DownloadData;
use server::handler::process_download_on_queue::ProcessDownloadOnQueueHandler;
use server::handler::Handler;

#[tokio::main]
async fn main() -> () {
    assert_ne!(Config::source_path(), "");
    assert_ne!(Config::target_path(), "");

    tracing_subscriber::fmt::init();

    let (tx, mut rx) = tokio::sync::mpsc::channel::<DownloadData>(100);
    tokio::spawn(async move {
        loop {
            match database::first().await {
                Some(game_download_data) => {
                    tx.send(game_download_data).await.unwrap();
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

                database::remove(&download_data.id.as_str()).await;
            }
            None => {}
        }
    });

    let app = App::new(8080);
    app.listen().await
}
