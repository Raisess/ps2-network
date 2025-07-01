use std::sync::OnceLock;

use redb::{Database, ReadableTable, TableDefinition};
use tokio::sync::Mutex;

use super::download_provider::DownloadData;

static DB: OnceLock<Mutex<Database>> = OnceLock::new();
static TABLE: TableDefinition<&str, String> = TableDefinition::new("ps2-network-server-db");

fn database() -> &'static Mutex<Database> {
    DB.get_or_init(|| Mutex::new(Database::create("data.redb").unwrap()))
}

pub async fn insert(key: &str, data: &DownloadData) -> () {
    let db = database().lock().await;
    let write_txn = db.begin_write().unwrap();

    {
        let mut write_table = write_txn.open_table(TABLE).unwrap();
        let json = serde_json::to_string(&data).unwrap();
        write_table.insert(key, json).unwrap();
    }

    write_txn.commit().unwrap();
}

pub async fn remove(key: &str) -> () {
    let db = database().lock().await;
    let write_txn = db.begin_write().unwrap();
    {
        let mut write_table = write_txn.open_table(TABLE).unwrap();
        write_table.remove(key).unwrap();
    }
    write_txn.commit().unwrap();
}

pub async fn list() -> Vec<DownloadData> {
    let db = database().lock().await;
    let read_txn = db.begin_read().unwrap();
    match read_txn.open_table(TABLE) {
        Ok(table) => table
            .iter()
            .unwrap()
            .map(|i| {
                let json = i.unwrap().1.value();
                serde_json::from_str::<DownloadData>(&json).unwrap()
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}

pub async fn exists(key: &str) -> bool {
    let db = database().lock().await;
    let read_txn = db.begin_read().unwrap();
    match read_txn.open_table(TABLE) {
        Ok(table) => match table.get(key).unwrap() {
            Some(_) => true,
            None => false,
        },
        Err(_) => false,
    }
}

pub async fn first() -> Option<DownloadData> {
    let db = database().lock().await;
    let read_txn = db.begin_read().unwrap();

    match read_txn.open_table(TABLE) {
        Ok(table) => match table.first().unwrap() {
            Some(row) => {
                drop(db);
                Some(serde_json::from_str::<DownloadData>(&row.1.value()).unwrap())
            }
            None => None,
        },
        Err(_) => None,
    }
}
