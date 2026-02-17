use crate::{errors::clap_missing_arg, prelude::mathing_proto::UserEditRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_edit(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_edit(args)?;
        let table: tabled::Table = self
            .connect()
            .await?
            .user_edit(req)
            .await?
            .into_inner()
            .user_row
            .ok_or(ServerError::NoneValue("UserEditResponse"))?
            .into();

        println!("{table}");
        Ok(())
    }
}

fn user_edit(args: UserArgs) -> Result<UserEditRequest, ClientError> {
    if args.action != CrudAction::Update {
        return Err(ClientError::CrudInvalid(CrudAction::Update, args.action));
    }
    let target = args
        .target
        .ok_or_else(|| clap_missing_arg("target"))?
        .to_string();
    let name = args
        .name
        .ok_or_else(|| clap_missing_arg("name"))?
        .to_string();

    Ok(UserEditRequest { target, name })
}

#[cfg(test)]
mod tests {
    use crate::errors::expected_error;

    use super::*;

    const TARGET: &str = "jon";
    const NAME: &str = "thing";

    #[test]
    /// Tests that a valid cli argument results
    /// in the expected UserEditRequest message
    fn test_edit() -> anyhow::Result<()> {
        let want = UserEditRequest {
            target: TARGET.into(),
            name: NAME.into(),
        };
        let args = UserArgs {
            action: CrudAction::Update,
            target: Some(TARGET.into()),
            name: Some(NAME.into()),
        };
        let got = user_edit(args)?;

        assert_eq!(dbg!(want), dbg!(got));
        Ok(())
    }
    #[test]
    /// Tests possible cli errors return the correct error.
    /// user_edit requires the UserArs to have the action: CrudAction::Update.
    fn test_edit_action_error() -> anyhow::Result<()> {
        let want = ClientError::CrudInvalid(CrudAction::Update, CrudAction::Create);
        let args = UserArgs {
            action: CrudAction::Create,
            target: Some(TARGET.into()),
            name: Some(NAME.into()),
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        };
        Ok(())
    }
    #[test]
    /// Tests possible cli errors return the correct error.
    /// user_edit requires the User arg to have a target and a name,
    /// both of which are optional in the the CLI parser, but the target
    /// should be reported missing frist.
    fn test_edit_target_error() -> anyhow::Result<()> {
        let want = ClientError::ClapError(clap_missing_arg("target"));
        let args = UserArgs {
            action: CrudAction::Update,
            target: None,
            name: None,
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
    #[test]
    /// Tests possible cli errors return the correct error.
    /// user_edit requires the User arg to have a target and a name,
    /// both of which are optional in the the CLI parser.
    fn test_edit_name_error() -> anyhow::Result<()> {
        let want = ClientError::ClapError(clap_missing_arg("name"));
        let args = UserArgs {
            action: CrudAction::Update,
            target: Some(TARGET.into()),
            name: None,
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
