use std::net::SocketAddr;

use crate::prelude::*;

pub mod prelude {
    pub use super::ServerEndpoint;
}

pub struct ServerEndpoint(pub SocketAddr);

impl ServerEndpoint {
    pub fn try_init() -> Result<Self, Status> {
        let key = "SERVER_URI";

        info!("Parsing env variable: {key}.");
        let addr = std::env::var(key).map_err(|_| ServerError::ConfigMissing(key))?;

        let addr = addr
            .parse()
            .map_err(|_| ServerError::InvalidEndpoint(addr))?;

        Ok(Self(addr))
    }
    pub fn try_get() -> Result<SocketAddr, ServerError> {
        let endpoint = CONFIG
            .get()
            .ok_or(ServerError::ConfigError("server endpoint"))?
            .endpoint
            .0;

        Ok(endpoint)
    }
}
