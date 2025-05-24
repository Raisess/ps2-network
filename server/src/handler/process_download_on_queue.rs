use std::io::Read;
use std::path::{Path, PathBuf};

use crate::common::config::Config;
use crate::common::http_client::HttpClient;
use crate::core::art_provider::internet_archive::InternetArchiveArtProvider;
use crate::core::art_provider::ArtProvider;
use crate::core::database;
use crate::core::download_provider::{DownloadData, DownloadStatus};

use super::Handler;

const COMPRESSED_FILE_EXT: &str = ".zip";
const MAXIMUM_CD_SIZE_BYTES: u64 = 700 * 1_000_000;

pub struct ProcessDownloadOnQueueHandler {
    pub download_data: DownloadData,
}

#[async_trait::async_trait]
impl Handler<()> for ProcessDownloadOnQueueHandler {
    // @TODO: handle errors to not explode everything and revert on failure
    async fn handle(&self) -> () {
        let mut game = self.download_data.clone();
        Self::process(&mut game).await;
    }
}

impl ProcessDownloadOnQueueHandler {
    #[async_recursion::async_recursion]
    async fn process(game: &mut DownloadData) -> () {
        let step = game.status.clone().unwrap_or(DownloadStatus::PENDING);
        tracing::info!(game.id, "processing: {}: {:?}", game.filename, step);

        match step {
            DownloadStatus::PENDING => {
                game.step(DownloadStatus::DOWNLOADING);
                database::insert(&game.id, &game).await;
            }
            DownloadStatus::DOWNLOADING => {
                tracing::info!(game.id, "downloading...");

                // @TODO: delete file if already exists
                let download_path = get_path_buf(vec![&Config::source_path(), &game.filename]);
                let _ = HttpClient::download(&game.url, &download_path.to_str().unwrap()).await;
                tracing::info!(game.id, "downloaded!");

                game.step(DownloadStatus::EXTRACTING);
                database::insert(&game.id, &game).await;
            }
            DownloadStatus::EXTRACTING => {
                tracing::info!(game.id, "extracting...");

                // @TODO: delete extracted files if already exists
                let download_path = get_path_buf(vec![&Config::source_path(), &game.filename]);
                let file_stream = std::fs::File::open(&download_path).unwrap();
                let download_path_parent = download_path.parent().unwrap();
                zip_extract::extract(file_stream, &download_path_parent, true).unwrap();

                std::fs::remove_file(&download_path).unwrap();
                tracing::info!(game.id, "extracted!");

                game.step(DownloadStatus::INSTALLING);
                database::insert(&game.id, &game).await;
            }
            DownloadStatus::INSTALLING => {
                tracing::info!(game.id, "installing...");

                let download_path = get_path_buf(vec![&Config::source_path(), &game.filename]);
                let extracted_paths = match_extracted_paths(&download_path);
                if extracted_paths.len() > 1 {
                    // @TODO: convert bin/cue to iso
                    // @REF: https://en.wikipedia.org/wiki/Cue_sheet_(computing)
                    // @REF: http://he.fi/bchunk/
                    tracing::error!(game.id, "ERROR: Don't support bin/cue conversion");
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
                    get_path_buf(vec![&Config::target_path(), &target_dir.to_string()]);
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
                    get_normalized_filename_for_opl(&game_serial, &game_filename);
                let destination_path = get_path_buf(vec![
                    target_dir_path.to_str().unwrap(),
                    &converted_game_filename,
                ]);

                // @TODO: delete file if already exists
                std::fs::copy(&extracted_path, &destination_path).unwrap();
                std::fs::remove_file(&extracted_path).unwrap();
                tracing::info!(game.id, "installed!");

                game.serial = Some(game_serial);
                game.step(DownloadStatus::DOWNLOADINGART);
                database::insert(&game.id, &game).await;
            }
            DownloadStatus::DOWNLOADINGART => {
                tracing::info!(game.id, "downloading ART...");

                let game_serial = game
                    .serial
                    .as_ref()
                    .expect("For some reason your download do not have a serial code");

                let art_provider = InternetArchiveArtProvider;
                let art_data = art_provider.get(&game_serial).await;
                let art_dir_path = get_path_buf(vec![&Config::target_path(), "ART"]);
                if !art_dir_path.is_dir() {
                    std::fs::create_dir(&art_dir_path).unwrap();
                }

                // @TODO: delete files if already exists
                let bg_art_path =
                    get_path_buf(vec![&art_dir_path.to_string_lossy(), &art_data.bg_file]);
                let _ =
                    HttpClient::download(&art_data.bg_url, &bg_art_path.to_string_lossy()).await;

                let cov_art_path =
                    get_path_buf(vec![&art_dir_path.to_string_lossy(), &art_data.cov_file]);
                let _ =
                    HttpClient::download(&art_data.cov_url, &cov_art_path.to_string_lossy()).await;

                let lgo_art_path =
                    get_path_buf(vec![&art_dir_path.to_string_lossy(), &art_data.lgo_file]);
                let _ =
                    HttpClient::download(&art_data.lgo_url, &lgo_art_path.to_string_lossy()).await;
                tracing::info!(game.id, "downloaded ART!");

                game.step(DownloadStatus::DONE);
                database::insert(&game.id, &game).await;
            }
            DownloadStatus::DONE => {
                database::remove(&game.id).await;
                tracing::info!(game.id, "done!");
                return ();
            }
        }

        Self::process(game).await;
    }
}

fn get_path_buf(path: Vec<&str>) -> PathBuf {
    let mut path_buf = PathBuf::from(path[0]);
    for i in 1..path.len() {
        path_buf.push(path[i]);
    }

    path_buf
}

fn get_normalized_filename_for_opl(game_serial: &String, filename: &String) -> String {
    let re = regex::Regex::new(r"\(([^)]+)\)").unwrap();
    let modified_filename = re.replace_all(&filename, "").replace(".iso", "");
    let game_filename = &modified_filename
        .trim()
        .chars()
        .take(32)
        .collect::<String>();
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
