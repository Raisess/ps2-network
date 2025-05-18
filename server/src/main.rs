use std::time::Duration;

use server::app::App;
use server::handler::process_download_on_queue::ProcessDownloadOnQueueHandler;
use server::handler::Handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::rt::spawn(async {
        loop {
            actix_web::rt::time::sleep(Duration::from_millis(1000)).await;
            let handler = ProcessDownloadOnQueueHandler;
            handler.handle().await;
        }
    });

    let app = App::new(8080);
    app.listen().await
}
