use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

use crate::core::game_download_provider::GameDownloadData;

pub fn queue() -> &'static Mutex<VecDeque<GameDownloadData>> {
    static ARRAY: OnceLock<Mutex<VecDeque<GameDownloadData>>> = OnceLock::new();
    ARRAY.get_or_init(|| Mutex::new(VecDeque::new()))
}
