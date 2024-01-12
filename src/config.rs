use std::collections::HashMap;
use serde::de::value;
use serde::{Deserialize, Serialize};
use config::{Config, File as ConfigFile};

// use serde_json::{Result as serdeResult, Value};
use std::fs::File;
use std::io::Write;

use crate::CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct MyConfig {
    pub jwt: String,
    pub paths: Vec<HashMap<String, String>>,
}
const CONFIG_PATH: &str = "config.json";
pub fn read_config() -> Result<MyConfig, std::convert::Infallible> {


    // Attempt to read the configuration file
    let config_result = Config::builder()
        .add_source(ConfigFile::with_name(CONFIG_PATH))
        .build();

    match config_result {
        Ok(config) => {
            let my_config: MyConfig = config.try_deserialize().unwrap();
            Ok(my_config)
        }
        Err(_) => {
            let default_config = MyConfig {
                jwt: String::from(""),
                paths: vec![HashMap::new()],
            };

            let default_config_json = serde_json::to_string(&default_config).unwrap();

            let mut file = File::create(CONFIG_PATH).unwrap();
            file.write_all(default_config_json.as_bytes()).unwrap();

            Ok(default_config)
        }
    }
}

pub fn append_path(new_path: HashMap<String, String>) {
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
        path.get(&key) != Some(&value)
    });
 
    let updated_config_json = serde_json::to_string(&*config).unwrap();
 
    let mut file = File::create(CONFIG_PATH).unwrap();
    file.write_all(updated_config_json.as_bytes()).unwrap();
}

pub fn check_path(config: &MyConfig, key: String, value: String) -> bool {
    for path in &config.paths {
        if path.contains_key(&key) && path.get(&key) == Some(&value) {
            return true;
        }
    }
    false
}