use crate::config::Config;
use aws_sdk_s3::primitives::ByteStream;
use notify::{event::CreateKind, Event, RecursiveMode, Result, Watcher};
use std::fs;
use std::path::Path;
use std::sync::mpsc;


/// Решение на винду
fn parse_path(path: &str) -> String {
    let uuids: Vec<&str> = path.split("\\").last().unwrap().split(".").collect();
    let (user_uuid, file_uuid) = (uuids[0], uuids[1]);
    format!("{}/{}", user_uuid, file_uuid)
}

pub async fn file_lisener_worker(config: &Config) -> Result<()> {
    if fs::metadata("tmp").is_err() {
        fs::create_dir("tmp").unwrap();
    }

    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(Path::new("./tmp"), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                if event.kind == notify::EventKind::Create(CreateKind::Any) {
                    let path = event.paths[0].to_str().unwrap();
                    println!("WORKER GET FILE - {:?}", path);

                    let parsed_path = parse_path(path);
                    println!("WORKER Parsed path: {}", parsed_path);

                    let body = ByteStream::from_path(std::path::Path::new(path)).await;
                    let _ = config
                        .s3_client
                        .put_object()
                        .bucket("objects")
                        .key(parsed_path)
                        .body(body.unwrap())
                        .send()
                        .await
                        .unwrap();

                    let remove_file = fs::remove_file(path);
                    println!("WORKER REMOVE FILE {:?}", remove_file);
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    Ok(())
}
