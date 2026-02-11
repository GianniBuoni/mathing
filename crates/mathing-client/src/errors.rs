use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Connection to endpoint '{0}' failed.")]
    Connection(Arc<str>),
    /// For when the client expects a message with an Optional field
    /// to return a Some value,
    /// but the server successfully returned a None value somehow.
    #[error("Server returned a None values for the message {0}.")]
    NoneValue(Arc<str>),
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error(
        "Configured client endpoint is missing. Please set with `SERVER_URI` enviorment variable"
    )]
    EndpointMissing,
    #[error(
        "Configured enviorment variable `SERVER_URI={0}` cannot be parsed as an endpoint string."
    )]
    EndpointInvalid(Arc<str>),
}
