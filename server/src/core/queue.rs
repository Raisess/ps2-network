use async_std::sync::Mutex;
use std::collections::VecDeque;
use std::sync::OnceLock;

use crate::core::download_provider::DownloadData;

static ARRAY: OnceLock<Mutex<VecDeque<DownloadData>>> = OnceLock::new();

pub fn queue() -> &'static Mutex<VecDeque<DownloadData>> {
    ARRAY.get_or_init(|| Mutex::new(VecDeque::new()))
}
