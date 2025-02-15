use std::borrow::Cow;

use anyhow::bail;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub api_address: Cow<'static, str>,
    pub jwt_secret: Cow<'static, str>,

    pub minio_url: Cow<'static, str>,
    pub upload_main_bucket: Cow<'static, str>,

    pub postgres_user: Cow<'static, str>,
    pub postgres_password: Cow<'static, str>,
    // todo: construct url
    pub database_url: Cow<'static, str>,
    pub database_pool_size: u32,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        // ---------------------- change to validator

        Ok(Self {
            // API
            api_address: match dotenv::var("API_ADDRESS") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing API_ADDRESS: {err}"),
            },
            jwt_secret: match dotenv::var("JWT_SECRET") {
                Ok(port) => port.into(),
                Err(err) => bail!("missing JWT_SECRET: {err}"),
            },
            // S3
            minio_url: match dotenv::var("MINIO_URL") {
                Ok(port) => port.into(),
                Err(err) => bail!("missing MINIO_URL: {err}"),
            },
            upload_main_bucket: match dotenv::var("UPLOAD_MAIN_BUCKET") {
                Ok(port) => port.into(),
                Err(err) => bail!("missing UPLOAD_MAIN_BUCKET: {err}"),
            },
            // DB
            postgres_user: match dotenv::var("POSTGRES_USER") {
                Ok(port) => port.into(),
                Err(err) => bail!("missing POSTGRES_USER: {err}"),
            },
            postgres_password: match dotenv::var("POSTGRES_PASSWORD") {
                Ok(port) => port.into(),
                Err(err) => bail!("missing POSTGRES_PASSWORD: {err}"),
            },
            database_url: match dotenv::var("DATABASE_URL") {
                Ok(port) => port.into(),
                Err(err) => bail!("missing DATABASE_URL: {err}"),
            },
            database_pool_size: match dotenv::var("DATABASE_POOL_SIZE") {
                Ok(port) => port.parse()?,
                Err(err) => bail!("missing DATABASE_POOL_SIZE: {err}"),
            },
        })
    }
}
