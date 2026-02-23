use crate::{
    errors::clap_missing_arg,
    prelude::mathing_proto::{UserEdit, UserEditRequest},
};

use super::*;

impl UserService {
    pub(super) async fn handle_edit(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_edit(args)?;
        let table = self
            .connect()
            .await?
            .user_edit(req)
            .await?
            .into_inner()
            .users
            .into_iter()
            .collect::<tabled::Table>();

        println!("{table}");
        Ok(())
    }
}

fn user_edit(args: UserArgs) -> Result<UserEditRequest, ClientError> {
    if args.action != CrudAction::Update {
        return Err(ClientError::CrudInvalid(CrudAction::Update, args.action));
    }
    if args.targets.is_empty() {
        return Err(ClientError::ClapError(clap_missing_arg("targets")));
    }
    if args.names.is_empty() {
        return Err(ClientError::ClapError(clap_missing_arg("names")));
    }
    if args.targets.len() != args.names.len() {
        return Err(ClientError::UnevenArgs(
            CrudAction::Update,
            "targets, names",
        ));
    }
    let user_edit = args
        .targets
        .into_iter()
        .zip(args.names)
        .map(|(target, name)| UserEdit { target, name })
        .collect();

    Ok(UserEditRequest { user_edit })
}

#[cfg(test)]
mod tests {
    use crate::{errors::expected_error, prelude::mathing_proto::UserEdit};

    use super::*;

    fn user_edit_arg() -> UserEdit {
        UserEdit {
            target: "jon".into(),
            name: "thing".into(),
        }
    }

    #[test]
    /// Tests that a valid cli argument results
    /// in the expected UserEditRequest message
    fn test_edit() -> anyhow::Result<()> {
        let want = UserEditRequest {
            user_edit: vec![user_edit_arg()],
        };
        let args = UserArgs {
            action: CrudAction::Update,
            targets: vec![user_edit_arg().target],
            names: vec![user_edit_arg().name],
        };
        let got = user_edit(args)?;

        assert_eq!(dbg!(want), dbg!(got));
        Ok(())
    }
    #[test]
    /// Tests possible cli errors returns the correct error.
    /// user_edit requires the UserArs to have the action: CrudAction::Update.
    fn test_action_error() {
        let want = ClientError::CrudInvalid(CrudAction::Update, CrudAction::Create);
        let args = UserArgs {
            action: CrudAction::Create,
            targets: vec![user_edit_arg().target],
            names: vec![user_edit_arg().name],
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        };
    }
    #[test]
    /// Tests possible cli errors return the correct error.
    /// user_edit requires the User arg to have a target and a name,
    /// both of which are optional in the the CLI parser, but the target
    /// should be reported missing frist.
    fn test_target_error() {
        let want = ClientError::ClapError(clap_missing_arg("targets"));
        let args = UserArgs {
            action: CrudAction::Update,
            targets: vec![],
            names: vec![],
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
    #[test]
    /// Tests possible cli errors return the correct error.
    /// user_edit requires the User arg to have a target and a name,
    /// both of which are optional in the the CLI parser.
    fn test_name_error() {
        let want = ClientError::ClapError(clap_missing_arg("names"));
        let args = UserArgs {
            action: CrudAction::Update,
            targets: vec![user_edit_arg().target],
            names: vec![],
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
    #[test]
    /// Cli can return args of unequal length
    fn test_uneven_len() {
        let want = ClientError::UnevenArgs(CrudAction::Update, "targets, names");
        let args = UserArgs {
            action: CrudAction::Update,
            targets: vec![user_edit_arg().target; 3],
            names: vec![user_edit_arg().name],
        };
        let got = user_edit(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
