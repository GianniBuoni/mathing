use crate::{errors, prelude::mathing_proto::UserCreateRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_create(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_create(args)?;
        let user = Into::<tabled::Table>::into(
            self.connect()
                .await?
                .user_create(req)
                .await?
                .into_inner()
                .user_row
                .ok_or(ServerError::NoneValue("UserCreateResponse"))?,
        );

        println!("{user}");
        Ok(())
    }
}

fn user_create(args: UserArgs) -> Result<UserCreateRequest, ClientError> {
    if args.action != CrudAction::Create {
        return Err(ClientError::CrudInvalid(CrudAction::Create, args.action));
    }
    let name = args
        .name
        .ok_or_else(|| errors::clap_missing_arg("name"))?
        .to_string();

    Ok(UserCreateRequest { name })
}

#[cfg(test)]
mod tests {
    use crate::errors::{clap_missing_arg, expected_error};

    use super::*;

    const NAME: &str = "jon";

    #[test]
    fn test_user_create() -> anyhow::Result<()> {
        let want = UserCreateRequest { name: NAME.into() };
        let args = UserArgs {
            action: CrudAction::Create,
            target: None,
            name: Some(NAME.into()),
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
            target: None,
            name: None,
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
            target: Some(NAME.into()),
            name: None,
        };
        let got = user_create(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
