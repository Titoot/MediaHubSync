use notify::{Config as notifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
use notify::event::{EventKind, ModifyKind, RenameMode};

use crate::{requests, config};

pub async fn watch_chunk(path: &config::Path) {
    log::info!("Watching {}", path.path);

    if let Err(error) = watch(path).await {
        log::error!("Error watching {}: {error:?}", path.path);
    }
}

async fn watch(path: &config::Path) -> notify::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, notifyConfig::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path.path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == EventKind::Modify(ModifyKind::Any) || event.kind == EventKind::Modify(ModifyKind::Name(RenameMode::To)) || EventKind::is_remove(&event.kind) {
                    log::info!("Change: {0:?}", event.paths[0]);
                    requests::sync(event.paths[0].to_str().unwrap(), &path.srv_path, &path.folder_type).await;
                }
            },
            Err(error) => log::error!("Error: {error:?}"),
        }
    }

    Ok(())
}