#![allow(dead_code)]
#![allow(unused)]

use axum::{
    routing::{get, post},
    Extension,
    Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};

use tokio::net::TcpListener;

use rust_file_share::{config::Config, logger, route::app};

#[derive(Clone)]
struct State {}

async fn foo() -> String {
    String::from("hello")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Arc::new(Config::load()?);
    logger::init(&config)?;
    let app = app().await?;
    dbg!("Running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
