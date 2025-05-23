use std::time::Duration;

use server::app::App;
use server::common::config::Config;
use server::core::queue::queue;
use server::handler::process_download_on_queue::ProcessDownloadOnQueueHandler;
use server::handler::Handler;

#[tokio::main]
async fn main() -> () {
    assert_ne!(Config::source_path(), "");
    assert_ne!(Config::target_path(), "");

    tokio::spawn(async {
        loop {
            // @TODO: remove queue and use mpsc channel with some persist data
            let clone_queue = queue().lock().unwrap().clone();
            match clone_queue.front() {
                Some(download_data) => {
                    let handler = ProcessDownloadOnQueueHandler {
                        download_data: download_data.clone(),
                    };
                    handler.handle().await;
                    queue().lock().unwrap().pop_front();
                }
                None => {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }
        }
    });

    let app = App::new(8080);
    app.listen().await
}
