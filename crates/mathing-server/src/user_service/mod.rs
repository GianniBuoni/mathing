use crate::prelude::*;

pub use mathing_proto::user_service_server::{UserService, UserServiceServer};
use mathing_proto::{
    UserCreateRequest, UserCreateResponse, UserDeleteRequest, UserDeleteResponse, UserGetRequest,
    UserGetResponse, UserListRequest, UserListResponse,
};

mod user_create;
mod user_delete;
mod user_get;
mod user_list;
mod user_row;

#[derive(Debug, Default)]
pub struct MathingUserService {}

#[tonic::async_trait]
impl UserService for MathingUserService {
    async fn user_create(
        &self,
        req: Request<UserCreateRequest>,
    ) -> Result<Response<UserCreateResponse>, Status> {
        self.handle_create(req).await
    }

    async fn user_delete(
        &self,
        req: Request<UserDeleteRequest>,
    ) -> Result<Response<UserDeleteResponse>, Status> {
        self.handle_delete(req).await
    }

    async fn user_get(
        &self,
        req: Request<UserGetRequest>,
    ) -> Result<Response<UserGetResponse>, Status> {
        self.handle_get(req).await
    }

    async fn user_list(
        &self,
        req: Request<UserListRequest>,
    ) -> Result<Response<UserListResponse>, Status> {
        self.handle_list(req).await
    }
}
