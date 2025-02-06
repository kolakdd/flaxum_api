use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::task;

use flaxum::utils::watcher;
use flaxum::{config::AppConfig, logger, routes::main_app::app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init()?;
    // load config
    let config = Arc::new(AppConfig::load().await?);
    let workers_config = config.clone();

    // spawn worker
    task::spawn(async move {
        let _ = watcher::file_lisener_worker(workers_config.clone()).await;
    });

    let listener = TcpListener::bind(config.env.api_address.to_string()).await?;
    tracing::info!("Server start's on {}", &config.env.api_address.to_string());

    let app = app().await?;
    axum::serve(listener, app).await?;
    Ok(())
}
