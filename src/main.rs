use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use std::sync::{Mutex, Arc};
use eframe::egui;

mod ui;
mod config;
mod watcher;
mod requests;

lazy_static! {
   static ref CONFIG: Mutex<config::MyConfig> = Mutex::new(config::read_config().unwrap());
}

#[tokio::main]
async fn main() {
        let rt = Runtime::new().unwrap();
        let rt = Arc::new(Mutex::new(rt));
     
        let rt_clone = Arc::clone(&rt);
        std::thread::spawn(move || {
            let rt = rt_clone.lock().unwrap();
            rt.block_on(async {
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
            });
        });
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("JWT: {}",  CONFIG.lock().unwrap().jwt);
    
    let win_option = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([330.0, 500.0]).with_resizable(false),
        ..Default::default()
    };
    let _ = eframe::run_native("MediaSubSync", win_option, Box::new(|_cc| Box::<ui::MyApp>::default()));



}
