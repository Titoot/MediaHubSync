use std::{path::Path, sync::Mutex};
use fs_extra::dir::get_size;
use lazy_static::lazy_static;
use itertools::Itertools;

use serde::{Serialize, Deserialize};
use widestring::U16CString;
use win_msgbox::Okay;

use crate::{CONFIG, config};

pub struct UiState {
    pub(crate) is_loading: bool,
}

impl UiState {
    pub fn set_loading(&mut self, loading: bool) {
        self.is_loading = loading;
    }
}
lazy_static! {
    pub static ref UI_STATE: Mutex<UiState> = Mutex::new(UiState {
        is_loading: false,
    });
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
   success: bool,
   message: String,
}

#[derive(Serialize)]
pub struct PathInfo {
   fileName: String,
   srvPath: String,
   size: u64,
   fileType: String,
}

pub async fn sync(paths: Vec<config::Path>) {
    let api_url = CONFIG.lock().unwrap().api_url.clone();
    if api_url.is_empty() {
        log::error!("api url not found in config file");
        let message = U16CString::from_str("Please add api url to config file").unwrap();
        let _ = win_msgbox::error::<Okay>(message.as_ptr()).show().unwrap();
        return;
    }

    UI_STATE.lock().unwrap().set_loading(true);

    let mut path_infos: Vec<PathInfo> = Vec::new();
    for p in paths {
        log::debug!("path: {}", p.path);
        let size = get_size(p.path.clone()).unwrap();
        log::debug!("File Size: {}", size);
        let path_info = PathInfo {
            fileName: Path::new(&p.path).file_name().unwrap().to_str().unwrap().to_string(),
            srvPath: p.srv_path,
            size: size,
            fileType: p.folder_type,
        };
        path_infos.push(path_info);
    }
    let json = serde_json::json!(&path_infos);

    let client = reqwest::Client::new();
    let res = client.post(format!("https://{}/folderSync", api_url))
        .header("Authorization", format!("Bearer {}",  CONFIG.lock().unwrap().jwt))
        .json(&json)
        .send()
        .await;

    UI_STATE.lock().unwrap().set_loading(false);

    match res {
        Ok(res) => {
            log::debug!("Status Code: {}", res.status());
            let json_result: Result<Response, _> = res.json().await;
            match json_result {
                Ok(json_value) => log::info!("{:?}", json_value),
                Err(e) => log::error!("Failed to deserialize JSON: {}", e),
            }
        },
        Err(e) => log::error!("Failed to send request: {}", e),
        }
}

pub async fn sync_all(path: config::Path){
    let paths = std::fs::read_dir(path.path).unwrap();
    let paths: Vec<config::Path> = paths.filter_map(|entry| {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => return None,
        };
        
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return None,
        };
        
        if metadata.is_dir() || 
           (metadata.is_file() && matches!(entry.path().extension(), Some(ext) if ext == "rar" || ext == "zip")) {
            Some(config::Path {
                path: entry.path().to_str().unwrap().to_string(),
                srv_path: path.srv_path.clone(),
                folder_type: path.folder_type.clone(),
            })
        } else {
            None
        }
    }).collect();
  
    let paths: Vec<config::Path> = paths.into_iter().unique().collect();
    
    sync(paths).await;
}