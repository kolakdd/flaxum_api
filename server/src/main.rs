use std::sync::Arc;

use flaxum::config::parameter;
use flaxum::routes::root::app;
use tokio::net::TcpListener;
use tokio::task;

use flaxum::utils::watcher;
use flaxum::{config::AppConfig, logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    parameter::init();
    logger::init()?;

    // load config
    let config = Arc::new(AppConfig::load().await?);
    let workers_config = config.clone();

    // spawn worker
    task::spawn(async move {
        let _ = watcher::file_lisener_worker(workers_config.clone()).await;
    });

    let listener = TcpListener::bind(config.env.api_address.to_string()).await?;

    let app = app(config.clone()).await;

    tracing::info!("Server start's on {}", &config.env.api_address.to_string());

    axum::serve(listener, app).await?;
    Ok(())
}
