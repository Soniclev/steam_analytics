use std::{fs, sync::Arc, time::{Duration, UNIX_EPOCH}};
use actix_web::rt::time::sleep;
use chrono::{TimeZone, Utc};

use crate::{import::import_item, AppStateWithCounter};

pub async fn import_items_from_folder(app_data: &Arc<AppStateWithCounter>, folder_path: &str) {
    // load mocked data from folder ./mocked
    // list all files in the folder
    let mut files = fs::read_dir(folder_path).unwrap();
    let mut imported: u64 = 0;
    while let Some(entry) = files.next() {
        #[cfg(debug_assertions)]
        const MAX_ITEMS: u64 = 25;
        #[cfg(not(debug_assertions))]
        const MAX_ITEMS: u64 = 30000;
        if imported >= MAX_ITEMS {
            break;
        }
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".html") {
                let file = fs::read_to_string(path.clone()).unwrap();
                let file_saved_at = path
                    .clone()
                    .metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                let date = Utc
                    .timestamp_millis_opt(file_saved_at.try_into().unwrap())
                    .unwrap();
                let item = import_item(&file, date);
                if let Some(item) = item {
                    {
                        let mut items = app_data.items.lock().unwrap();

                        let market_name = item.name.clone();
                        items.insert(market_name.clone(), item);
                        let mut items_to_process = app_data.items_to_process.lock().unwrap();
                        items_to_process.push_back(market_name);
                        imported += 1;

                    }
                    
                    // switch asyncio context
                    if imported % 100 == 0 {
                        sleep(Duration::from_millis(1)).await;
                    }
                }
            }
        }
    }
}
