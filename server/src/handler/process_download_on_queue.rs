use crate::common::http_client::HttpClient;
use crate::core::queue::queue;

use super::Handler;

const MAXIMUM_CD_SIZE_BYTES: u64 = 700 * 1_000_000;

pub struct ProcessDownloadOnQueueHandler;

#[async_trait::async_trait]
impl Handler<()> for ProcessDownloadOnQueueHandler {
    async fn handle(&self) -> () {
        let game = queue().lock().unwrap().pop_front();

        match game {
            Some(game) => {
                println!("game: {:#?}", game);

                let download_path =
                    std::path::PathBuf::from(format!("/home/danilo/Downloads/{}", game.filename));
                println!("{}", &download_path.to_str().unwrap());
                let _ = HttpClient::download(&game.url, &download_path.to_str().unwrap()).await;
                println!("DONE!");

                let file = std::fs::read(&download_path).unwrap();
                zip_extract::extract(std::io::Cursor::new(file), download_path.parent().unwrap(), true).unwrap();

                //std::fs::remove_file(&download_path).unwrap();
                // @TODO: rename the file to GAME_ID.name.iso pattern

                let target_dir =
                    if std::fs::metadata(&download_path).unwrap().len() > MAXIMUM_CD_SIZE_BYTES {
                        "DVD"
                    } else {
                        "CD"
                    };
                let destination_path = std::path::PathBuf::from(format!(
                    "/home/danilo/temp_target/{target_dir}/{}",
                    game.filename
                ));
                std::fs::rename(download_path, destination_path).unwrap();

                // @TODO: download game art
                // should do that directly on the target directory???
            }
            None => {}
        }
    }
}
