use crate::prelude::mathing_proto::UserCreateRequest;

use super::*;

impl UserService {
    pub(super) async fn handle_create(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = tonic::Request::new(UserCreateRequest {
            name: args.name.to_string(),
        });
        let res = self.connect().await?.user_create(req).await?.into_inner();
        println!("RESPONSE: {:?}", res);

        Ok(())
    }
}
