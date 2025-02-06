use aws_sdk_s3::Client;

use crate::config::database::DB;
use crate::config::env::EnvironmentVariables;
use crate::config::AppConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: DB,
    pub s3: Client,
    pub env: EnvironmentVariables,
}

impl AppState {
    pub async fn build(config: AppConfig) -> AppState {
        AppState {
            db: config.db_pool,
            s3: config.s3_client,
            env: config.env,
        }
    }
}
