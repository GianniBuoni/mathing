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
    /// Error for unsucessful type casing usually from Pg and Proto types
    /// that can onlys implement `TryFrom`.
    ///
    /// Usage:
    /// ```
    /// use mathing_server::prelude::ServerError;
    ///
    /// let source = "Decimal";
    /// let target = "f32";
    /// let value = 1.49.to_string();
    ///
    /// ServerError::ConversionError(source, target, value);
    /// ```
    #[error("Type casting failed: Source type: '{0}', Target type: '{1}', Value: '{2}'")]
    ConversionError(&'static str, &'static str, String),
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
        "Client error: server cannot edit or delete arguments that do not exist within the database: Table: '{0}', Value(s): '{1}'."
    )]
    EntryNotFound(String, String),
    /// Client sent arguments that have repeated elements.
    /// These can be problematic for db tables that have unique constrains.
    #[error("Client error: server does not accept arguemnts with repeated value(s): '{0}'")]
    RpeatedValue(String),
    /// Client sent arguments that are already in the database.
    /// This error should be returned if the table has unique contraints.
    #[error(
        "Client error: Table: '{0}' expects unique value(s). Arg(s) already present within database: '{1}'."
    )]
    UniqueConstraint(String, String),
    #[error("Cient error: Unable to parse '{0}' as a uuid(s)")]
    Uuid(String),
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
