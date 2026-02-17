use crate::{errors, prelude::mathing_proto::UserDeleteRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_delete(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = UserDeleteRequest {
            target: args
                .name
                .ok_or_else(|| errors::clap_missing_arg("target"))?
                .to_string(),
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
