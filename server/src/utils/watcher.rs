use crate::config::Config;
use aws_sdk_s3::primitives::ByteStream;
use notify::{event::CreateKind, Event, RecursiveMode, Result, Watcher};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc;

fn parse_path(path: &str) -> String {
    let uuids: Vec<&str> = path.split("\\").last().unwrap().split(".").collect();
    let (user_uuid, file_uuid) = (uuids[0], uuids[1]);
    format!("{}/{}", user_uuid, file_uuid)
}

pub async fn file_lisener_worker(config: Arc<Config>) -> Result<()> {
    if fs::metadata("tmp").is_err() {
        fs::create_dir("tmp").unwrap();
    }

    let (tx, mut rx) = mpsc::channel::<Event>(100); // Асинхронный канал с буфером на 100 событий
    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            let _ = tx.blocking_send(event); // Отправляем событие в канал
        }
    })?;
    watcher.watch(Path::new("./tmp"), RecursiveMode::NonRecursive)?;

    while let Some(event) = rx.recv().await {
        if event.kind == notify::EventKind::Create(CreateKind::Any) {
            let config = Arc::clone(&config);
            let path = event.paths[0].to_str().unwrap().to_string();
            tokio::spawn(async move {
                let parsed_path = parse_path(&path);

                let body = ByteStream::from_path(std::path::Path::new(&path)).await;
                if let Err(e) = body {
                    println!("Error reading file: {:?}", e);
                    return;
                }

                let result = config
                    .s3_client
                    .put_object()
                    .bucket("objects")
                    .key(parsed_path)
                    .body(body.unwrap())
                    .send()
                    .await;

                match result {
                    Ok(response) => println!("S3 upload success: {:?}", response),
                    Err(e) => println!("S3 upload error: {:?}", e),
                }

                if let Err(e) = fs::remove_file(&path) {
                    println!("Error removing file: {:?}", e);
                }
            });
        }
    }

    Ok(())
}
