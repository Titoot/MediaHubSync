use std::collections::HashMap;

use crate::CONFIG;

pub async fn sync(path: &str, srv_path: &str) {
    let json = &serde_json::json!({
            "path": path,
            "srvPath": srv_path,
    });

    let client = reqwest::Client::new();
    let res = client.post("https://enbm3oqrsc8c4.x.pipedream.net")
        .header("Authorization", format!("Bearer {}",  CONFIG.lock().unwrap().jwt))
        .json(json)
        .send()
        .await;

    match res {
        Ok(res) => {
            let json_result: Result<HashMap<String, String>, _> = res.json().await;
            match json_result {
                Ok(json_value) => println!("{:?}", json_value),
                Err(e) => println!("Failed to deserialize JSON: {}", e),
            }
        },
        Err(e) => println!("Failed to send request: {}", e),
    }
}