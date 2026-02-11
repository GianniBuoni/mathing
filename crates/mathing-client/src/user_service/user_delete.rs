use crate::prelude::mathing_proto::{OneOfId, UserDeleteRequest, one_of_id};

use super::*;

impl UserService {
    pub(super) async fn handle_delete(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = UserDeleteRequest {
            one_of_id: Some(OneOfId {
                one_of_id: Some(one_of_id::OneOfId::Name(args.name.to_string())),
            }),
        };
        let rows = self
            .connect()
            .await?
            .user_delete(req)
            .await?
            .into_inner()
            .rows_affected
            .ok_or(ServerError::NoneValue("UserDeleteResponse".into()))?
            .rows_affected;

        println!("Rows deleted: {rows}.");

        Ok(())
    }
}
