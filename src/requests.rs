use std::collections::HashMap;
use fs_extra::dir::get_size;

use widestring::U16CString;
use win_msgbox::Okay;

use crate::CONFIG;

pub async fn sync(path: &str, srv_path: &str) {
    let api_url = CONFIG.lock().unwrap().api_url.clone();
    if api_url.is_empty() {
        log::error!("api url not found in config file");
        let message = U16CString::from_str("Please add api url to config file").unwrap();
        let _ = win_msgbox::error::<Okay>(message.as_ptr()).show().unwrap();
        return;
    }
    let size = get_size(path).unwrap();
    log::debug!("Fize Size: {}", size);
    let json = &serde_json::json!({
            "path": path,
            "srvPath": srv_path,
            "size": size,
    });

    let client = reqwest::Client::new();
    let res = client.post(format!("https://{}/", api_url))
        .header("Authorization", format!("Bearer {}",  CONFIG.lock().unwrap().jwt))
        .json(json)
        .send()
        .await;

    match res {
        Ok(res) => {
            let json_result: Result<HashMap<String, String>, _> = res.json().await;
            match json_result {
                Ok(json_value) => log::info!("{:?}", json_value),
                Err(e) => log::error!("Failed to deserialize JSON: {}", e),
            }
        },
        Err(e) => log::error!("Failed to send request: {}", e),
    }
}