use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::{Result as serdeResult, Value};
use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;
use config::{Config, File as ConfigFile};
use notify::{Config as notifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{EventKind, ModifyKind, RenameMode};

#[derive(Debug, Deserialize, Serialize)]
struct MyConfig {
    jwt: String,
    paths: Vec<HashMap<String, String>>,
}

fn read_config() -> Result<MyConfig, std::convert::Infallible> {
    let config = Config::builder()
        .add_source(ConfigFile::with_name("config.json"))
        .build();

    let my_config: MyConfig = config.unwrap().try_deserialize().unwrap();

    Ok(my_config)
}

// fn append_path_chunk(new_chunk: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
//     let mut config = Config::builder()
//         .add_source(ConfigFile::with_name("config.json"))
//         .build()?;

//     let mut my_config: MyConfig = config.try_deserialize()?;

//     my_config.paths.extend(new_chunk.clone());
//     CONFIG.lock().unwrap().paths.extend(new_chunk);

//     let serialized_config = serde_json::to_string_pretty(&my_config)?;

//     let mut file = File::create("config.json")?;
//     file.write_all(serialized_config.as_bytes())?;

    

//     Ok(())
// }

use std::sync::Mutex;

lazy_static! {
   static ref CONFIG: Mutex<MyConfig> = Mutex::new(read_config().unwrap());
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("JWT: {}",  CONFIG.lock().unwrap().jwt);

    // let new_paths = vec!["D:\\GamesFiles".to_string(), "/Titoot/Games".to_string()];
    // append_path_chunk(new_paths).unwrap();

    let mut tasks = vec![];
    for path_map in &CONFIG.lock().unwrap().paths {
        for (key, value) in path_map {
            println!("Path: {} -> Value: {}", key, value);

            let key = key.clone();
            let value = value.clone();
            let task = tokio::spawn(async move { watch_chunk(&key, &value).await });
            tasks.push(task);
        }
    }

    futures::future::join_all(tasks).await;
}

async fn watch_chunk(srv_path: &String, path: &String) {
    log::info!("Watching {}", path);

    if let Err(error) = watch(path, srv_path).await {
        log::error!("Error watching {}: {error:?}", path);
    }
}

async fn watch<P: AsRef<Path>>(path: P, srv_path: &String) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, notifyConfig::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Modify(ModifyKind::Any) || event.kind == EventKind::Modify(ModifyKind::Name(RenameMode::To)) || EventKind::is_remove(&event.kind) {
                    log::info!("Change: {0:?}", event.paths[0]);
                    sync(event.paths[0].to_str().unwrap(), srv_path).await;
                }
            },
            Err(error) => log::error!("Error: {error:?}"),
        }
    }

    Ok(())
}

async fn sync(path: &str, srv_path: &str) {
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