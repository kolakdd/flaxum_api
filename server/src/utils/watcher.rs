use crate::config::Config;
use notify::{event::CreateKind, Event, RecursiveMode, Result, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc;
pub async fn file_lisener_worker(config: &Config) -> Result<()> {
    let folder_exist = fs::metadata("tmp").is_ok();
    if !folder_exist {
        fs::create_dir("tmp").unwrap();
    }
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new("./tmp"), RecursiveMode::NonRecursive)?;
    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == notify::EventKind::Create(CreateKind::Any) {
                    println!("created - : {:?}", event.paths[0]);
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
