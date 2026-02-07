pub mod prelude {
    pub mod mathing_proto {
        tonic::include_proto!("mathing");
    }
    pub use super::cli::*;
    pub use super::user_service::UserService;
}

mod cli;
pub mod errors;
mod user_service;
