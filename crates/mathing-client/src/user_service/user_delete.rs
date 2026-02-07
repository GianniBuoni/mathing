use crate::prelude::mathing_proto::UserDeleteRequest;

use super::*;

impl UserService {
    pub(super) async fn handle_delete(&self, _args: UserArgs) -> anyhow::Result<()> {
        // match name to uuid or return server error
        let req = UserDeleteRequest { uuid: 1 };
        let res = self.connect().await?.user_delete(req).await?.into_inner();
        println!("RESPONSE: {:?}", res);

        Ok(())
    }
}
