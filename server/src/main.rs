use std::time::Duration;

use server::app::App;
use server::common::config::Config;
use server::handler::process_download_on_queue::ProcessDownloadOnQueueHandler;
use server::handler::Handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    assert_ne!(Config::source_path(), "");
    assert_ne!(Config::target_path(), "");

    actix_web::rt::spawn(async {
        loop {
            actix_web::rt::time::sleep(Duration::from_millis(100)).await;
            let handler = ProcessDownloadOnQueueHandler;
            handler.handle().await;
        }
    });

    let app = App::new(8080);
    app.listen().await
}
