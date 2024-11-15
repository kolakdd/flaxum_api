#![allow(dead_code)]
#![allow(unused)]

use std::fs;
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sqlx::PgPool;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use tokio::net::TcpListener;

use rust_file_share::{config::Config, logger, route::app};

#[derive(Clone)]
struct State {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Arc::new(Config::load()?);
    fs::create_dir("tmp")?;
    logger::init(&config)?;
    let app = app().await?;
    dbg!("Running on http://localhost:3000");
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
