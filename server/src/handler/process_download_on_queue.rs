use std::path::{Path, PathBuf};

use crate::common::http_client::HttpClient;
use crate::core::download_provider::DownloadStatus;
use crate::core::queue::queue;

use super::Handler;

const COMPRESSED_FILE_EXT: &str = ".zip";
const MAXIMUM_CD_SIZE_BYTES: u64 = 700 * 1_000_000;

pub struct ProcessDownloadOnQueueHandler;

#[async_trait::async_trait]
impl Handler<()> for ProcessDownloadOnQueueHandler {
    async fn handle(&self) -> () {
        // @TODO: update game state before poping and pop only at the end process
        let mut queue = queue().lock().await;

        match queue.front_mut() {
            Some(game) => {
                println!("game: {:#?}", game);
                game.status = Some(DownloadStatus::DOWNLOADING);

                let download_path =
                    PathBuf::from(format!("/home/danilo/Downloads/{}", game.filename));
                println!("{}", &download_path.to_str().unwrap());
                let _ = HttpClient::download(&game.url, &download_path.to_str().unwrap()).await;
                println!("DONE!");

                game.status = Some(DownloadStatus::EXTRACTING);

                let file = std::fs::read(&download_path).unwrap();
                let download_path_parent = download_path.parent().unwrap();
                zip_extract::extract(std::io::Cursor::new(file), &download_path_parent, true)
                    .unwrap();

                std::fs::remove_file(&download_path).unwrap();

                game.status = Some(DownloadStatus::INSTALLING);

                let extracted_paths = match_extracted_paths(&download_path);
                if extracted_paths.len() > 1 {
                    // @TODO: convert bin/cue to iso
                    eprintln!("ERROR: Don't support bin/cue conversion");
                    return ();
                }

                let extracted_path = extracted_paths[0].clone();
                let target_dir =
                    if std::fs::metadata(&extracted_path).unwrap().len() > MAXIMUM_CD_SIZE_BYTES {
                        "DVD"
                    } else {
                        "CD"
                    };

                let game_filename = extracted_path.file_name().unwrap().to_str().unwrap();
                // @TODO: get game GAME_SERIAL
                // @TODO: rename the file to GAME_SERIAL.name.iso pattern using the opl maximum size
                let converted_game_filename = game_filename;
                let destination_path = PathBuf::from(format!(
                    "/home/danilo/temp_target/{target_dir}/{}",
                    converted_game_filename,
                ));
                std::fs::rename(extracted_path, destination_path).unwrap();

                // @TODO: download game art
                // should do that directly on the target directory???
                game.status = Some(DownloadStatus::COMPLETED);
                queue.pop_front();
            }
            None => {}
        }
    }
}

fn match_extracted_paths(download_path: &Path) -> Vec<PathBuf> {
    let filename = download_path
        .to_str()
        .unwrap()
        .replace(COMPRESSED_FILE_EXT, "");

    let parent_path = download_path.parent().unwrap();
    parent_path
        .read_dir()
        .unwrap()
        .filter_map(|file| {
            let file_to_match = file.unwrap().path();
            let file_to_match_str = file_to_match.to_str().unwrap();
            let file_to_match_str_wo_ext =
                &file_to_match_str[..file_to_match_str.len() - COMPRESSED_FILE_EXT.len()];
            if file_to_match_str_wo_ext == filename {
                return Some(file_to_match);
            }

            None
        })
        .collect()
}
