pub use crate::prelude::mathing_proto::item_service_server::ItemServiceServer;
use crate::prelude::{
    mathing_proto::{ItemCreateRequest, ItemCreateResponse},
    *,
};

mod item_create;
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
}
