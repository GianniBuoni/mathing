use std::time::Duration;

use sqlx::PgPool;

use crate::prelude::*;

pub mod prelude {
    pub use super::DBconn;
}

pub struct DBconn(PgPool);

impl DBconn {
    pub async fn try_init() -> Result<Self, Status> {
        let key = "DATABASE_URL";

        info!("Parsing env variable: {key}.");
        let url = std::env::var(key).map_err(|_| ServerError::ConfigMissing(key))?;

        info!("Establishing connection with database endpoint: {url}");

        let pool = tokio::time::timeout(Self::context(), PgPool::connect(&url))
            .await
            .map_err(|_| DbError::ContextError)?
            .map_err(|_| DbError::ConnectionError(url))?;

        info!("Database connection successful; server ready to make SQL queries.");
        Ok(Self(pool))
    }

    pub async fn try_get() -> Result<&'static PgPool, ServerError> {
        Ok(&CONFIG.get().ok_or(ServerError::ConfigError("DB"))?.store.0)
    }

    pub fn context() -> Duration {
        Duration::from_secs(10)
    }
}
