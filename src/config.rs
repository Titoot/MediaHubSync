use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use config::{Config, File as ConfigFile};

use serde_json::{Result as serdeResult, Value};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize, Serialize)]
pub struct MyConfig {
    pub jwt: String,
    pub paths: Vec<HashMap<String, String>>,
}

pub fn read_config() -> Result<MyConfig, std::convert::Infallible> {
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