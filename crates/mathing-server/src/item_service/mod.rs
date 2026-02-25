pub use crate::prelude::mathing_proto::item_service_server::ItemServiceServer;
use crate::prelude::{
    mathing_proto::{
        ItemCreateRequest, ItemCreateResponse, ItemDeleteRequest, ItemDeleteResponse,
        ItemEditRequest, ItemEditResponse, ItemGetRequest, ItemGetResponse, ItemListRequest,
        ItemListResponse,
    },
    *,
};

mod item_create;
mod item_delete;
mod item_edit;
mod item_get;
mod item_list;
mod item_row;

#[derive(Debug, Default)]
pub struct MathingItemService {}

#[tonic::async_trait]
impl mathing_proto::item_service_server::ItemService for MathingItemService {
    async fn item_create(
        &self,
        req: Request<ItemCreateRequest>,
    ) -> Result<Response<ItemCreateResponse>, Status> {
        self.handle_create(req).await
    }
    async fn item_delete(
        &self,
        req: Request<ItemDeleteRequest>,
    ) -> Result<Response<ItemDeleteResponse>, Status> {
        self.handle_delete(req).await
    }
    async fn item_edit(
        &self,
        req: Request<ItemEditRequest>,
    ) -> Result<Response<ItemEditResponse>, Status> {
        self.handle_edit(req).await
    }
    async fn item_get(
        &self,
        req: Request<ItemGetRequest>,
    ) -> Result<Response<ItemGetResponse>, Status> {
        self.handle_get(req).await
    }
    async fn item_list(
        &self,
        req: Request<ItemListRequest>,
    ) -> Result<Response<ItemListResponse>, Status> {
        self.handle_list(req).await
    }
}
