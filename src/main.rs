use std::{env, process};
use lazy_static::lazy_static;
use std::sync::Mutex;
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
   let DEBUG: bool = env::args().any(|arg| arg == "--debug");
   if !DEBUG {
       hide_console_window();
   }
   env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(if DEBUG { "debug" } else { "info" })).init();
   log::debug!("Running in debug mode");

   tokio::spawn(async move {

        let paths = CONFIG.lock().unwrap().paths.clone();
        let tasks: Vec<_> = paths.into_iter().map(|path| {
            log::info!("Path: {} -> SrvPath: {}", path.path, path.srv_path);
            tokio::spawn(async move { watcher::watch_chunk(&path).await })
        }).collect();

       futures::future::join_all(tasks).await;
   });

   log::debug!("JWT: {}", CONFIG.lock().unwrap().jwt);
   
   let win_option = eframe::NativeOptions {
       viewport: egui::ViewportBuilder::default().with_inner_size([363.0, 500.0]).with_resizable(false),
       ..Default::default()
   };
   let run_result = eframe::run_native("MediaHubSync", win_option, Box::new(|cc| {
       egui_extras::install_image_loaders(&cc.egui_ctx);

       Box::<ui::MyApp>::default()
   }));

   match run_result {
       Ok(_) => {
            log::info!("Application closed normally");
            process::exit(0);
        },
       Err(e) => log::info!("Application closed with error: {:?}", e),
   }

}


fn hide_console_window() {
    unsafe { winapi::um::wincon::FreeConsole() };
}
