use crate::{errors::clap_missing_arg, prelude::mathing_proto::UserDeleteRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_delete(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_delete(args)?;
        let rows = Into::<tabled::Table>::into(
            self.connect()
                .await?
                .user_delete(req)
                .await?
                .into_inner()
                .rows_affected
                .ok_or(ServerError::NoneValue("UserDeleteResponse"))?,
        );

        println!("{rows}");
        Ok(())
    }
}

fn user_delete(args: UserArgs) -> Result<UserDeleteRequest, ClientError> {
    if args.action != CrudAction::Delete {
        return Err(ClientError::CrudInvalid(CrudAction::Delete, args.action));
    }
    if args.targets.is_empty() {
        return Err(ClientError::ClapError(clap_missing_arg("target")));
    }
    Ok(UserDeleteRequest {
        targets: args.targets,
    })
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    fn targets() -> Vec<String> {
        vec!["jon".into()]
    }

    #[test]
    fn test_user_delete() -> anyhow::Result<()> {
        let want = UserDeleteRequest { targets: targets() };
        let args = UserArgs {
            action: CrudAction::Delete,
            targets: targets(),
            names: vec![],
        };
        let got = user_delete(args)?;

        assert_eq!(dbg!(want), dbg!(got));
        Ok(())
    }
    #[test]
    /// user_delete should only accept CrudAction::Delete
    fn test_action_error() -> anyhow::Result<()> {
        let want = ClientError::CrudInvalid(CrudAction::Delete, CrudAction::Update);
        let args = UserArgs {
            action: CrudAction::Update,
            targets: vec![],
            names: vec![],
        };
        let got = user_delete(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
    #[test]
    /// user_delete should only accept target arguments,
    /// any other argument should be ignored
    fn test_arg_error() -> anyhow::Result<()> {
        let want = ClientError::ClapError(clap_missing_arg("target"));
        let args = UserArgs {
            action: CrudAction::Delete,
            targets: vec![],
            names: targets(),
        };
        let got = user_delete(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
