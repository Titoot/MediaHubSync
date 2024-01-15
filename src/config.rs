use serde::{Deserialize, Serialize};
use config::{Config, File as ConfigFile};
use widestring::U16CString;
use win_msgbox::Okay;

// use serde_json::{Result as serdeResult, Value};
use std::fs::File;
use std::io::Write;

use crate::CONFIG;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    fn new() -> Self {
        Self {
            path: String::new(),
            srv_path: String::new(),
            folder_type: String::new(),
        }
    }
 }

 impl From<(String, String, String)> for Path {
    fn from((path, srv_path, folder_type): (String, String, String)) -> Self {
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
    CONFIG.lock().unwrap().paths.push(new_path);
 
    let updated_config_json = serde_json::to_string(&*CONFIG.lock().unwrap()).unwrap();
 
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();
}

pub fn set_jwt(new_jwt: String) {
    CONFIG.lock().unwrap().jwt = new_jwt;
 
    let updated_config_json = serde_json::to_string(&*CONFIG.lock().unwrap()).unwrap();
 
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();
}

pub fn delete_path(config: &mut MyConfig, key: String, value: String) {
    config.paths.retain(|path| {
        !(path.path == key && path.srv_path == value)
    });
  
    let updated_config_json = serde_json::to_string(&*config).unwrap();
  
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();
 }
 
 pub fn check_path(config: &MyConfig, key: String, value: String) -> bool {
    for path in &config.paths {
        if path.path == key && path.srv_path == value {
            return true;
        }
    }
    false
 }