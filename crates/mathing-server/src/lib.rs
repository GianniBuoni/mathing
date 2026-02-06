pub mod prelude {
    // Tonic wrapper types all messages should use.
    pub use tonic::{Request, Response, Status};
    // Generated module that provides all message and service types
    pub mod mathing_proto {
        tonic::include_proto!("mathing");
    }
}
pub use user_service::{MathingUserService, UserServiceServer};

mod user_service;
