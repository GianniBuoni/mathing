use sqlx::PgPool;

use crate::prelude::*;

pub mod prelude {
    pub use super::DBconn;
}

pub struct DBconn(PgPool);

impl DBconn {
    pub async fn try_init() -> anyhow::Result<Self> {
        let key = "DATABASE_URL";

        info!("Parsing env variable: {key}.");
        let url = std::env::var(key).map_err(|_| ServerError::ConfigMissing(key.into()))?;

        info!("Establishing connection with database endpoint: {url}");
        let pool = PgPool::connect(url.as_str())
            .await
            .map_err(|_| DbError::ConnectionError(url))?;

        info!("Database connection successful; server ready to make SQL queries.");
        Ok(Self(pool))
    }
    pub async fn try_get() -> anyhow::Result<&'static PgPool, Status> {
        Ok(&CONFIG
            .get()
            .ok_or(ServerError::ConfigError("DB".into()))
            .map_err(|e| Status::unavailable(e.to_string()))?
            .store
            .0)
    }
}
