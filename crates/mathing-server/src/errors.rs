use thiserror::Error;

pub mod prelude {
    pub use super::{DbError, ServerError};
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(
        "Server {0} getter called on uninitialized server configuration. Check if conifguration initialization has been called."
    )]
    ConfigError(String),
    #[error("Required env variable {0} is missing.")]
    ConfigMissing(String),
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("DB endpoint '{0}' is offline or invalid; no conncection could be initialized.")]
    ConnectionError(String),
}
