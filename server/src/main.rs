use std::sync::Arc;

use tokio::net::TcpListener;
// use tokio::task;

// use flaxum::utils::watcher;
use flaxum::{config::Config, logger, route::app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Arc::new(Config::load().await?);

    // let worker_config = config.clone();
    // task::spawn(async move {
    //     let _ = watcher::file_lisener_worker(&worker_config).await;
    // });

    logger::init(&config)?;
    let app = app().await?;
    dbg!("Running on http://localhost:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
