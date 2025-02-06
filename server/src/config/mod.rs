pub mod database;
pub mod env;
pub mod s3;

use crate::config::env::EnvironmentVariables;
use anyhow;
use aws_sdk_s3::Client;
use database::Database;
use s3::S3Client;
use sqlx::{Pool, Postgres};

pub const SIZE_1GB: usize = 1024 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub env: EnvironmentVariables,
    pub db_pool: Pool<Postgres>,
    pub s3_client: Client,
}

impl AppConfig {
    pub async fn load() -> anyhow::Result<Self> {
        let env = EnvironmentVariables::from_env()?;
        let db_pool = Database::init(&env).await?.pool;
        let s3_client = S3Client::init(&env).await;
        Ok(Self {
            env,
            db_pool,
            s3_client,
        })
    }
}
