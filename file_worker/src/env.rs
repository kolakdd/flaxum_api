use std::borrow::Cow;

use anyhow::bail;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub minio_url: Cow<'static, str>,
    pub upload_main_bucket: Cow<'static, str>,

    pub postgres_user: Cow<'static, str>,
    pub postgres_password: Cow<'static, str>,

    // todo: construct url
    pub database_url: Cow<'static, str>,
    pub database_pool_size: u32,

    pub rmq_host: Cow<'static, str>,
    pub rmq_port: u16,
    pub rmq_user: Cow<'static, str>,
    pub rmq_pass: Cow<'static, str>,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        // ---------------------- change to validator
        Ok(Self {
            // S3
            minio_url: match dotenv::var("MINIO_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing MINIO_URL: {err}"),
            },
            upload_main_bucket: match dotenv::var("UPLOAD_MAIN_BUCKET") {
                Ok(bucket) => bucket.into(),
                Err(err) => bail!("missing UPLOAD_MAIN_BUCKET: {err}"),
            },
            // DB
            postgres_user: match dotenv::var("POSTGRES_USER") {
                Ok(user) => user.into(),
                Err(err) => bail!("missing POSTGRES_USER: {err}"),
            },
            postgres_password: match dotenv::var("POSTGRES_PASSWORD") {
                Ok(pass) => pass.into(),
                Err(err) => bail!("missing POSTGRES_PASSWORD: {err}"),
            },
            database_url: match dotenv::var("DATABASE_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing DATABASE_URL: {err}"),
            },
            database_pool_size: match dotenv::var("DATABASE_POOL_SIZE") {
                Ok(pool_size) => pool_size.parse()?,
                Err(err) => bail!("missing DATABASE_POOL_SIZE: {err}"),
            },
            // RMQ
            rmq_host: match dotenv::var("RABBITMQ_HOST") {
                Ok(rmq_host) => rmq_host.into(),
                Err(err) => bail!("missing RABBITMQ_HOST: {err}"),
            },
            rmq_port: match dotenv::var("RABBITMQ_PORT") {
                Ok(port) => port.parse()?,
                Err(err) => bail!("missing RABBITMQ_PORT: {err}"),
            },
            rmq_user: match dotenv::var("RABBITMQ_DEFAULT_USER") {
                Ok(rmq_user) => rmq_user.into(),
                Err(err) => bail!("missing RABBITMQ_DEFAULT_USER: {err}"),
            },
            rmq_pass: match dotenv::var("RABBITMQ_DEFAULT_PASS") {
                Ok(rmq_password) => rmq_password.into(),
                Err(err) => bail!("missing RABBITMQ_DEFAULT_PASS: {err}"),
            },
        })
    }
}
