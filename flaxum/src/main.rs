use std::sync::Arc;

use flaxum::config::parameter;
use flaxum::routes::root::app;
use tokio::net::TcpListener;
use tokio::task;

use flaxum::{config::AppConfig, logger};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    parameter::init();
    logger::init()?;
    let config = Arc::new(AppConfig::load().await?);
    task::spawn(async move {
        tracing::warn!("worker spawn activation");
        file_worker::spawn_worker().await;
    });
    let listener = TcpListener::bind(config.env.api_address.to_string()).await?;
    let app = app(config.clone()).await;
    tracing::info!("Server start's on {}", &config.env.api_address.to_string());
    axum::serve(listener, app).await?;
    Ok(())
}
