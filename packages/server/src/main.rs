use server::app::App;
use server::common::config::Config;
use server::core::database;
use server::handler::process_download_on_queue::ProcessDownloadOnQueueHandler;
use server::handler::Handler;

#[tokio::main]
async fn main() -> () {
    assert_ne!(Config::source_path(), "");
    assert_ne!(Config::target_path(), "");

    tracing_subscriber::fmt::init();

    tokio::spawn(async move {
        let previous_added_downloads = database::list().await;
        for download_data in previous_added_downloads {
            let handler = ProcessDownloadOnQueueHandler { download_data };
            handler.handle().await;
        }
    });

    tokio::spawn(async move {
        loop {
            match database::first().await {
                Some(download_data) => {
                    let handler = ProcessDownloadOnQueueHandler { download_data };
                    handler.handle().await;
                }
                None => {}
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    });

    let app = App::new(8080);
    app.listen().await
}
