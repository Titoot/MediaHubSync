use lazy_static::lazy_static;
use std::sync::Mutex;

mod config;
mod watcher;
mod requests;

lazy_static! {
   static ref CONFIG: Mutex<config::MyConfig> = Mutex::new(config::read_config().unwrap());
}


slint::include_modules!();

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
            let task = tokio::spawn(async move { watcher::watch_chunk(&key, &value).await });
            tasks.push(task);
        }
    }

    futures::future::join_all(tasks).await;

    let ui = MainWindow::new().unwrap();

    // let ui_handle = ui.as_weak();
    // ui.on_show_popup(move || {
    //     let ui = ui_handle.unwrap();
    // });

    ui.run();


}