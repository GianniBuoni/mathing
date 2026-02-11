use crate::prelude::mathing_proto::{OneOfId, UserDeleteRequest, one_of_id};

use super::*;

impl UserService {
    pub(super) async fn handle_delete(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = UserDeleteRequest {
            one_of_id: Some(OneOfId {
                one_of_id: Some(one_of_id::OneOfId::Name(args.name.to_string())),
            }),
        };
        let rows = Into::<tabled::Table>::into(
            self.connect()
                .await?
                .user_delete(req)
                .await?
                .into_inner()
                .rows_affected
                .ok_or(ServerError::NoneValue("UserDeleteResponse".into()))?,
        );

        println!("{rows}");

        Ok(())
    }
}
