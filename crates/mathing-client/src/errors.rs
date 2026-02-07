use std::sync::Arc;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Connection to endpoint '{0}' failed.")]
    Connection(Arc<str>),
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
