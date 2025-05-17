use std::collections::VecDeque;
use std::sync::{Mutex, OnceLock};

use crate::core::game_download_provider::GameDownloadData;

static ARRAY: OnceLock<Mutex<VecDeque<GameDownloadData>>> = OnceLock::new();

pub fn queue() -> &'static Mutex<VecDeque<GameDownloadData>> {
    ARRAY.get_or_init(|| Mutex::new(VecDeque::new()))
}
