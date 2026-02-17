#[cfg(test)]
use std::fmt::Debug;
use std::sync::Arc;

use crate::cli::CrudAction;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Connection to endpoint '{0}' failed.")]
    Connection(Arc<str>),
    /// For when the client expects a message with an Optional field
    /// to return a Some value,
    /// but the server successfully returned a None value somehow.
    #[error("Server returned a None values for the message {0}.")]
    NoneValue(&'static str),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("Action invalid: function accepts {0:?}, but got {1:?}")]
    /// For when a function expects a specific CRUD action,
    /// but got a different one.
    ///
    /// Usage:
    ///
    /// ```
    /// use mathing_client::prelude::CrudAction;
    /// use mathing_client::errors::ClientError;
    ///
    /// let want = CrudAction::Create;
    /// let got = CrudAction::Delete;
    /// let err = (ClientError::CrudInvalid(want, got));
    /// ```
    CrudInvalid(CrudAction, CrudAction),
    #[error(transparent)]
    /// Newtype wrapper for a clap error
    ClapError(#[from] clap::Error),
    #[error(
        "Configured client endpoint is missing. Please set with `SERVER_URI` enviorment variable"
    )]
    EndpointMissing,
    #[error(
        "Configured enviorment variable `SERVER_URI={0}` cannot be parsed as an endpoint string."
    )]
    EndpointInvalid(Arc<str>),
}

/// This function returns a clap fromatted error for missing required arguments.
/// Some clap arguments accept Optionals, but the EnumValues they are
/// combined with expect a Some value.
pub fn clap_missing_arg(value: &str) -> clap::Error {
    let mut err = clap::Error::new(clap::error::ErrorKind::MissingRequiredArgument);
    let arg = format!("<{}>", value.to_uppercase());
    err.insert(
        clap::error::ContextKind::Usage,
        clap::error::ContextValue::StyledStr(arg.into()),
    );
    err
}

#[cfg(test)]
/// Maps an Ok(value) or Some(value) to an error.
/// Used in tests where errors were expected,
/// but the function somehow succeeded.
pub fn expected_error(value: impl Debug) -> anyhow::Error {
    let message = format!("Test expected an error, but returned: {value:?}");
    anyhow::Error::msg(message)
}
