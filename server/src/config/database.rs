use sqlx::{Error, Pool, Postgres};

use crate::{
    entity::user::User,
    repository::user_repository::{UserRepository, UserRepositoryTrait},
};

use super::env::EnvironmentVariables;
use std::{sync::Arc, time::Duration};

use sqlx::{self, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

pub trait DatabaseTrait {
    fn init(
        env_var: &EnvironmentVariables,
    ) -> impl std::future::Future<Output = Result<Self, Error>> + Send
    where
        Self: Sized;

    fn get_pool(&self) -> &Pool<Postgres>;
}

impl DatabaseTrait for Database {
    /// Init database
    async fn init(env_var: &EnvironmentVariables) -> Result<Self, Error> {
        let pool = Self::connect(&env_var.database_url, env_var.database_pool_size).await?;
        let database = Database { pool };
        let _ = Self::migrate(&database.pool).await;
        let _ = Self::init_superuser(&database, env_var).await;

        Ok(database)
    }
    /// Get pool
    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl Database {
    /// Return database pool
    async fn connect(url: &str, pool_size: u32) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
            .connect(url)
            .await
            .map_err(|err| {
                tracing::error!("{}", err);
                err
            })
    }

    /// Migrate database
    async fn migrate(db: &Pool<Postgres>) -> Result<(), Error> {
        tracing::info!("Migrating database...");
        match sqlx::migrate!("db/migrations").run(db).await {
            Ok(()) => Ok(()),
            Err(err) => {
                tracing::error!("{}", &err);
                println!("Migrate Error... {err}");
                Err(err)
            }
        }?;
        tracing::info!("Successfully migrated!...");
        Ok(())
    }

    async fn init_superuser(db: &Self, env_var: &EnvironmentVariables) -> Result<(), Error> {
        tracing::info!("Init Superuser...");
        let user_repo = UserRepository::new(&Arc::new(db.clone()));
        let super_user = user_repo
            .select_by_email(env_var.flaxum_super_user_email.to_string())
            .await;
        match super_user {
            None => {
                let payload = User::build_superuser(env_var).await;
                let _ = futures::executor::block_on(user_repo.insert(payload))
                    .unwrap_or_else(|e| panic!("{e}"));
                tracing::info!("Superuser initializated success!");
            }
            Some(_) => {
                tracing::warn!("Superuser already exist!");
            }
        };
        Ok(())
    }
}
