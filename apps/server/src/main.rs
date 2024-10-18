#![allow(dead_code)]
#![allow(unused)]

use axum::{
    // extract::Multipart,
    routing::{get, post},
    Extension,
    Router,
};
use rust_file_share::domain::{auth, object, user};
use rust_file_share::route::app;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct State {}

async fn foo() -> String {
    String::from("hello")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = app().await?;
    dbg!("Running on http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
