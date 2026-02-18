#[cfg(test)]
use std::fmt::Debug;

use log::error;
use thiserror::Error;
use tonic::Status;

pub mod prelude {
    pub use super::{ClientError, DbError, ServerError};
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(
        "Server {0} getter called on uninitialized server configuration. Check if conifguration initialization has been called."
    )]
    ConfigError(&'static str),
    #[error("Required env variable {0} is missing.")]
    ConfigMissing(&'static str),
    #[error("Configured endpoint: '{0}' is invalid.")]
    InvalidEndpoint(String),
}

impl From<ServerError> for Status {
    fn from(value: ServerError) -> Self {
        error!("{value}");
        Status::internal(value.to_string())
    }
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("DB endpoint '{0}' is offline or invalid; no conncection could be initialized.")]
    ConnectionError(String),
    #[error("DB connection failed: Context deadline exceeded.")]
    ContextError,
    #[error("DB operation failed: Server expeted argements, but none were passed.")]
    EmptyArgs,
    #[error("DB entry not found: Table: {0}, value: {1}.")]
    EntryNotFound(&'static str, String),
    #[error("DB operation failed: {0}")]
    PgError(#[from] sqlx::Error),
    #[error("DB operation failed: Table: {0}, key: {1} must be unique.")]
    UniqueConstraint(&'static str, &'static str),
    #[error("DB operation failed: Unable to parse {0} as a uuid")]
    Uuid(String),
}

impl From<DbError> for Status {
    fn from(value: DbError) -> Self {
        error!("{value}");
        match value {
            DbError::ConnectionError(_) => Status::unavailable(value.to_string()),
            DbError::ContextError => Status::deadline_exceeded(value.to_string()),
            _ => Status::internal(value.to_string()),
        }
    }
}

#[derive(Debug, Error)]
pub enum ClientError {
    /// Client successfully sent a malformed request to the server,
    /// most likely due to Optional fields set in the protobuf messages.
    #[error("Client error: field(s) {0} not set")]
    MissingField(&'static str),
}

impl From<ClientError> for Status {
    fn from(value: ClientError) -> Self {
        error!("{value}");
        Status::invalid_argument(value.to_string())
    }
}

#[cfg(test)]
pub fn expected_error(value: impl Debug) -> anyhow::Error {
    let message = format!("Test expected an error but returned {value:?}");
    anyhow::Error::msg(message)
}
