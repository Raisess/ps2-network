use std::io::Read;
use std::path::{Path, PathBuf};

use crate::common::config::Config;
use crate::common::http_client::HttpClient;
use crate::core::queue::queue;

use super::Handler;

const COMPRESSED_FILE_EXT: &str = ".zip";
const MAXIMUM_CD_SIZE_BYTES: u64 = 700 * 1_000_000;

pub struct ProcessDownloadOnQueueHandler;

#[async_trait::async_trait]
impl Handler<()> for ProcessDownloadOnQueueHandler {
    async fn handle(&self) -> () {
        let clone_queue = queue().lock().unwrap().clone();
        let game = clone_queue.front();

        match game {
            Some(game) => {
                println!("Processing: {:#?}", game);
                let download_path =
                    PathBuf::from(format!("{}/{}", Config::source_path(), game.filename));
                let _ = HttpClient::download(&game.url, &download_path.to_str().unwrap()).await;

                let file = std::fs::read(&download_path).unwrap();
                let download_path_parent = download_path.parent().unwrap();
                zip_extract::extract(std::io::Cursor::new(file), &download_path_parent, true)
                    .unwrap();

                std::fs::remove_file(&download_path).unwrap();

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
                let target_dir_path =
                    PathBuf::from(format!("{}/{target_dir}", Config::target_path()));
                if !target_dir_path.is_dir() {
                    std::fs::create_dir(&target_dir_path).unwrap();
                }

                let game_serial = extract_game_serial_from_iso(&extracted_path);
                let game_filename = extracted_path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                let converted_game_filename =
                    get_normalized_filename_for_opl(game_serial, game_filename);
                let destination_path = PathBuf::from(format!(
                    "{}/{}",
                    target_dir_path.to_str().unwrap(),
                    converted_game_filename,
                ));
                std::fs::rename(extracted_path, destination_path).unwrap();

                // @TODO: download game art
                // should do that directly on the target directory???

                queue().lock().unwrap().pop_front();
                println!("Done!");
            }
            None => {}
        }
    }
}

fn get_normalized_filename_for_opl(game_serial: String, filename: String) -> String {
    // @TODO: remove (US) from the name
    let game_filename = &filename.replace(".iso", "")[..32];
    format!("{game_serial}.{game_filename}.iso")
}

fn extract_game_serial_from_iso(extracted_path: &Path) -> String {
    let file = std::fs::File::open(extracted_path).unwrap();
    let iso = cdfs::ISO9660::new(file).unwrap();

    let mut buffer = String::new();
    if let Some(cdfs::DirectoryEntry::File(file)) = iso.open("SYSTEM.CNF").unwrap() {
        file.read().read_to_string(&mut buffer).unwrap();
    }

    let line = buffer.split("\n").collect::<Vec<&str>>()[0];
    let pattern = regex::Regex::new(r"([A-Z]{4}_\d+\.\d+)").unwrap();
    let captures = pattern.captures(line).unwrap();

    captures.get(0).unwrap().as_str().to_string()
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
