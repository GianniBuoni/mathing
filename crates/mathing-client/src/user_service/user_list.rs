use crate::prelude::mathing_proto::UserListRequest;

use super::*;

impl UserService {
    pub(super) async fn handle_list(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_list(args)?;
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

fn user_list(args: UserArgs) -> Result<UserListRequest, ClientError> {
    if args.action != CrudAction::List {
        return Err(ClientError::CrudInvalid(CrudAction::List, args.action));
    }
    Ok(UserListRequest {})
}

#[cfg(test)]
mod test {
    use crate::errors::expected_error;

    use super::*;

    #[test]
    /// user_list should still validate if the correct action was passed to it.
    /// Any extra argument passed in should be ignored
    fn test_action_error() {
        let want = ClientError::CrudInvalid(CrudAction::List, CrudAction::Get);
        let args = UserArgs {
            action: CrudAction::Get,
            targets: vec![],
            names: vec![],
        };
        let got = user_list(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
