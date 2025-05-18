use std::io::Read;
use std::path::{Path, PathBuf};

use crate::common::config::Config;
use crate::common::http_client::HttpClient;
use crate::core::art_provider::internet_archive::InternetArchiveArtProvider;
use crate::core::art_provider::ArtProvider;
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

                println!("Downloading...");
                let download_path = get_path(&Config::source_path(), &game.filename);
                let _ = HttpClient::download(&game.url, &download_path.to_str().unwrap()).await;
                println!("Downloaded!");

                println!("Extracting...");
                let file = std::fs::read(&download_path).unwrap();
                let download_path_parent = download_path.parent().unwrap();
                zip_extract::extract(std::io::Cursor::new(file), &download_path_parent, true)
                    .unwrap();

                std::fs::remove_file(&download_path).unwrap();
                println!("Extracted!");

                println!("Installing...");
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
                let target_dir_path = get_path(&Config::target_path(), &target_dir.to_string());
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
                let destination_path = PathBuf::from(format!(
                    "{}/{}",
                    target_dir_path.to_str().unwrap(),
                    converted_game_filename,
                ));
                std::fs::rename(extracted_path, destination_path).unwrap();
                println!("Installed!");

                println!("Downlading ART...");
                let art_provider = InternetArchiveArtProvider;
                let art_data = art_provider.get(game_serial.as_str()).await;
                let art_dir_path = get_path(&Config::target_path(), &"ART".to_string());
                if !art_dir_path.is_dir() {
                    std::fs::create_dir(&art_dir_path).unwrap();
                }

                let bg_art_path =
                    format!("{}/{game_serial}_BG.png", art_dir_path.to_string_lossy());
                let _ = HttpClient::download(&art_data.bg_url, &bg_art_path).await;

                let cov_art_path =
                    format!("{}/{game_serial}_COV.png", art_dir_path.to_string_lossy());
                let _ = HttpClient::download(&art_data.cov_url, &cov_art_path).await;

                let logo_art_path =
                    format!("{}/{game_serial}_LGO.png", art_dir_path.to_string_lossy());
                let _ = HttpClient::download(&art_data.logo_url, &logo_art_path).await;
                println!("Downladed ART!");

                queue().lock().unwrap().pop_front();
                println!("Done!");
            }
            None => {}
        }
    }
}

fn get_path(dirname: &String, subdirname: &String) -> PathBuf {
    PathBuf::from(format!("{dirname}/{subdirname}"))
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
