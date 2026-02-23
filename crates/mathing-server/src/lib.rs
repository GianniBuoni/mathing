pub mod prelude {
    /// Postgres bind limit for bulk staements
    pub const BIND_LIMIT: usize = 65535;
    pub use std::sync::Arc;
    // Tonic wrapper types all messages should use.
    pub use tonic::{Request, Response, Status};
    /// Generated module that provides all message and service types
    pub mod mathing_proto {
        tonic::include_proto!("mathing");
    }
    pub use super::config::prelue::*;
    pub use super::db_conn::prelude::*;
    pub use super::endpoint::prelude::*;
    pub use super::errors::prelude::*;
    pub use super::logger::logger_init;
    pub use super::user_service::{MathingUserService, UserServiceServer};
    pub use super::validation::Validation;
    // Logging macros
    pub use log::{error, info, warn};
}

mod config;
mod db_conn;
mod endpoint;
mod errors;
mod get_duplicates;
mod logger;
mod user_service;
mod validation;
