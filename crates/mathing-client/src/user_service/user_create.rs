use crate::{errors::clap_missing_arg, prelude::mathing_proto::UserCreateRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_create(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_create(args)?;
        let table = self
            .connect()
            .await?
            .user_create(req)
            .await?
            .into_inner()
            .users
            .into_iter()
            .collect::<tabled::Table>();

        println!("{table}");
        Ok(())
    }
}

fn user_create(args: UserArgs) -> Result<UserCreateRequest, ClientError> {
    if args.action != CrudAction::Create {
        return Err(ClientError::CrudInvalid(CrudAction::Create, args.action));
    }
    if args.names.is_empty() {
        return Err(ClientError::ClapError(clap_missing_arg("name")));
    }
    Ok(UserCreateRequest { names: args.names })
}

#[cfg(test)]
mod tests {
    use crate::errors::{clap_missing_arg, expected_error};

    use super::*;

    fn names() -> Vec<String> {
        vec!["jon".into()]
    }

    #[test]
    fn test_user_create() -> anyhow::Result<()> {
        let want = UserCreateRequest { names: names() };
        let args = UserArgs {
            action: CrudAction::Create,
            targets: vec![],
            names: names(),
        };
        let got = user_create(args)?;

        assert_eq!(dbg!(want), dbg!(got));
        Ok(())
    }

    #[test]
    /// user_cerate should only accept CrudAction::Create
    fn test_action_error() -> anyhow::Result<()> {
        let want = ClientError::CrudInvalid(CrudAction::Create, CrudAction::List);
        let args = UserArgs {
            action: CrudAction::List,
            targets: vec![],
            names: vec![],
        };
        let got = user_create(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[test]
    /// user_create exepects the name arg to have a value
    fn test_name_error() -> anyhow::Result<()> {
        let want = ClientError::ClapError(clap_missing_arg("name"));
        let args = UserArgs {
            action: CrudAction::Create,
            targets: names(),
            names: vec![],
        };
        let got = user_create(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
