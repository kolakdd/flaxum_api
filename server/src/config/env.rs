use std::borrow::Cow;

use anyhow::bail;

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub api_address: Cow<'static, str>,
    pub jwt_secret: Cow<'static, str>,

    pub flaxum_super_user_email: Cow<'static, str>,
    pub flaxum_super_user_password: Cow<'static, str>,

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
                Ok(secret) => secret.into(),
                Err(err) => bail!("missing JWT_SECRET: {err}"),
            },
            flaxum_super_user_email: match dotenv::var("FLAXUM_SUPER_USER_EMAIL") {
                Ok(email) => email.into(),
                Err(err) => bail!("missing FLAXUM_SUPER_USER_EMAIL: {err}"),
            },
            flaxum_super_user_password: match dotenv::var("FLAXUM_SUPER_USER_PASSWORD") {
                Ok(pass) => pass.into(),
                Err(err) => bail!("missing FLAXUM_SUPER_USER_PASSWORD: {err}"),
            },
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
        })
    }
}
