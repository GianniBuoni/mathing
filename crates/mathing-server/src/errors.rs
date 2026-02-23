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
    /// Error for when the database experiences issues with a query
    /// despite valid argument were passed to it.
    #[error("DB operation failed: {0}")]
    PgError(#[from] sqlx::Error),
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
    #[error("Client error: server expected arguments, but none were given.")]
    EmptyArgs,
    /// Client is atempting to edit or delete arguments that do not exist.
    #[error(
        "Client error: server cannot edit or delete arguments that do not exist within the databse: Table: '{0}', value(s): '{1}'."
    )]
    EntryNotFound(String, String),
    /// Client sent arguments that have repeated elements.
    /// These can be problematic for db tables that have unique constrains.
    #[error("Client error: server does not accept arguemnts with repeated value(s): '{0}'")]
    RpeatedValue(String),
    /// Client sent arguments that are already in the database.
    /// This error should be returned if the table has unique contraints.
    #[error(
        "Client error: Table: '{0}' expects unique value(s), '{1}' already present in database."
    )]
    UniqueConstraint(String, String),
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
