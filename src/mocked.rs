use std::{fs, time::UNIX_EPOCH};

use actix_web::web;
use chrono::{TimeZone, Utc};

use crate::{import::import_item, AppStateWithCounter};

pub fn import_items_from_folder(app_data: &web::Data<AppStateWithCounter>, folder_path: &str) {
    // load mocked data from folder ./mocked
    // list all files in the folder
    let mut files = fs::read_dir(folder_path).unwrap();
    while let Some(entry) = files.next() {
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
                    let mut items = app_data.items.lock().unwrap();
                    items.insert(item.name.clone(), item);
                }
            }
        }
    }
}
