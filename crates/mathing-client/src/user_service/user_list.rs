use crate::prelude::mathing_proto::UserListRequest;

use super::*;

impl UserService {
    pub(super) async fn handle_list(&self) -> anyhow::Result<()> {
        let req = tonic::Request::new(UserListRequest::default());
        let users = self
            .connect()
            .await?
            .user_list(req)
            .await?
            .into_inner()
            .users;

        let users = tabled::Table::from_iter(users);
        println!("{users}");

        Ok(())
    }
}
