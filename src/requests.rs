use std::path::Path;
use fs_extra::dir::get_size;

use serde::{Serialize, Deserialize};
use widestring::U16CString;
use win_msgbox::Okay;

use crate::{CONFIG, config};

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
    let paths: Vec<config::Path> = paths.map(|entry| {
        let entry = entry.unwrap();
        config::Path {
            path: entry.path().to_str().unwrap().to_string(),
            srv_path: path.srv_path.clone(),
            folder_type: path.folder_type.clone(),
        }
    }).collect();
    sync(paths).await;
 }