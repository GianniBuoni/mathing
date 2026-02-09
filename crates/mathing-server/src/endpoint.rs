use std::net::SocketAddr;

use crate::prelude::*;

pub mod prelude {
    pub use super::ServerEndpoint;
}

pub struct ServerEndpoint(pub SocketAddr);

impl ServerEndpoint {
    pub fn try_init() -> anyhow::Result<Self> {
        let key = "SERVER_URI";

        info!("Parsing env variable: {key}.");
        let addr = std::env::var(key)
            .map_err(|_| ServerError::ConfigMissing(key.into()))?
            .parse()?;

        Ok(Self(addr))
    }
    pub fn try_get() -> Result<SocketAddr, ServerError> {
        let endpoint = CONFIG
            .get()
            .ok_or(ServerError::ConfigError("server endpoint".into()))?
            .endpoint
            .0;

        Ok(endpoint)
    }
}
