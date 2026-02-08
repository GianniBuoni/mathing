use sqlx::PgPool;

use crate::prelude::{CONFIG, DbError, ServerError};

pub mod prelude {
    pub use super::DBconn;
}

pub struct DBconn(PgPool);

impl DBconn {
    pub async fn try_init() -> anyhow::Result<Self> {
        let key = "DATABASE_URL";
        let url = std::env::var(key).map_err(|_| ServerError::ConfigMissing(key.into()))?;
        let pool = PgPool::connect(url.as_str())
            .await
            .map_err(|_| DbError::ConnectionError(url))?;

        Ok(Self(pool))
    }
    pub async fn try_get() -> anyhow::Result<&'static PgPool, ServerError> {
        Ok(&CONFIG
            .get()
            .ok_or(ServerError::ConfigError("DB".into()))?
            .store
            .0)
    }
}
