use tokio::sync::OnceCell;

use crate::prelude::*;

pub mod prelue {
    pub use super::{CONFIG, ServerConfig};
}

pub static CONFIG: OnceCell<ServerConfig> = OnceCell::const_new();

pub struct ServerConfig {
    pub store: DBconn,
    pub endpoint: ServerEndpoint,
}

impl ServerConfig {
    /// Initailizes all static configurations for the server.
    /// Does not return a struct; use the getters for each field to get a sepcific config component.
    pub async fn try_init() -> anyhow::Result<()> {
        info!("Initializing server configuration.");
        let config = async || {
            anyhow::Ok(Self {
                store: DBconn::try_init().await?,
                endpoint: ServerEndpoint::try_init()?,
            })
        };
        CONFIG.get_or_try_init(config).await?;

        info!("Initialization successful.");
        Ok(())
    }
}
