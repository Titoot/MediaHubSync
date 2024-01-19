use serde::{Deserialize, Serialize};
use config::{Config, File as ConfigFile};
use widestring::U16CString;
use win_msgbox::Okay;

use std::fs::File;
use std::io::Write;

use crate::{CONFIG, requests};

#[derive(Debug, Deserialize, Serialize, Clone, Hash, Eq, PartialEq)]
pub struct Path {
   pub path: String,
   pub srv_path: String,
   pub folder_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MyConfig {
   pub api_url: String,
   pub jwt: String,
   pub paths: Vec<Path>,
}

impl Path {
    // pub fn new() -> Self {
    //     Self {
    //         path: String::new(),
    //         srv_path: String::new(),
    //         folder_type: String::new(),
    //     }
    // }
    pub fn from((path, srv_path, folder_type): (String, String, String)) -> Self {
        Self {
            path,
            srv_path,
            folder_type,
        }
    }
}

impl MyConfig {
    fn new() -> Self {
        Self {
            api_url: String::new(),
            jwt: String::new(),
            paths: Vec::new(),
        }
    }
}
const CONFIG_PATH: &str = "config.json";
pub fn read_config() -> Result<MyConfig, Box<dyn std::error::Error>> {


    // Attempt to read the configuration file
    let config_result = Config::builder()
        .add_source(ConfigFile::with_name(CONFIG_PATH))
        .build();

    match config_result {
        Ok(config) => {
            match config.try_deserialize() {
                Ok(my_config) => Ok(my_config),
                Err(e) => {
                   let message = U16CString::from_str(format!("Failed to Read Config File: {}", e)).unwrap();
                   let _ = win_msgbox::error::<Okay>(message.as_ptr()).show().unwrap();
                   Err(Box::new(e))
                }
            }
            
        }
        Err(_) => {
            let default_config = MyConfig::new();

            let default_config_json = serde_json::to_string(&default_config).unwrap();

            let mut file = File::create(CONFIG_PATH).unwrap();
            file.write_all(default_config_json.as_bytes()).unwrap();

            Ok(default_config)
        }
    }
}

pub fn append_path(new_path: Path) {
    CONFIG.lock().unwrap().paths.push(new_path.clone());
 
    let updated_config_json = serde_json::to_string(&*CONFIG.lock().unwrap()).unwrap();
 
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();

    tokio::spawn(async move {
        requests::sync_all(new_path).await;
    });
}

pub fn set_jwt(new_jwt: String) {
    CONFIG.lock().unwrap().jwt = new_jwt;
 
    let updated_config_json = serde_json::to_string(&*CONFIG.lock().unwrap()).unwrap();
 
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();
}

pub fn delete_path(config: &mut MyConfig, srv_path: String, inpath: String) {
    config.paths.retain(|path| {
        !(path.srv_path == srv_path && path.path == inpath)
    });
  
    let updated_config_json = serde_json::to_string(&*config).unwrap();
  
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();
 }
 
 pub fn check_path(config: &MyConfig, srv_path: String, inpath: String) -> bool {
    for path in &config.paths {
        if path.srv_path == srv_path && path.path == inpath {
            return true;
        }
    }
    false
 }