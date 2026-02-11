use crate::prelude::mathing_proto::UserCreateRequest;

use super::*;

impl UserService {
    pub(super) async fn handle_create(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = tonic::Request::new(UserCreateRequest {
            name: args.name.to_string(),
        });
        let user = Into::<tabled::Table>::into(
            self.connect()
                .await?
                .user_create(req)
                .await?
                .into_inner()
                .user_row
                .ok_or(ServerError::NoneValue("UserCreateResponse".into()))?,
        );

        println!("{user}");

        Ok(())
    }
}
