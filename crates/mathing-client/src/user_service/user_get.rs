use crate::{errors, prelude::mathing_proto::UserGetRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_get(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = tonic::Request::new(UserGetRequest {
            name: args
                .name
                .ok_or_else(|| errors::clap_missing_arg("name"))?
                .to_string(),
        });
        let user = self
            .connect()
            .await?
            .user_get(req)
            .await?
            .into_inner()
            .user
            .ok_or(ServerError::NoneValue("UserGetResponse"))
            .map(Into::<tabled::Table>::into)?;

        println!("{user}");

        Ok(())
    }
}
