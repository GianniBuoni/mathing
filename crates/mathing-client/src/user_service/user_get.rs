use crate::{errors::clap_missing_arg, prelude::mathing_proto::UserGetRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_get(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_get(args)?;
        let users = self
            .connect()
            .await?
            .user_get(req)
            .await?
            .into_inner()
            .users
            .into_iter()
            .collect::<tabled::Table>();

        println!("{users}");
        Ok(())
    }
}

fn user_get(args: UserArgs) -> Result<UserGetRequest, ClientError> {
    if args.action != CrudAction::Get {
        return Err(ClientError::CrudInvalid(CrudAction::Get, args.action));
    }
    if args.targets.is_empty() {
        return Err(ClientError::ClapError(clap_missing_arg("targets")));
    }
    Ok(UserGetRequest {
        names: args.targets,
    })
}

#[cfg(test)]
mod tests {
    use crate::errors::{clap_missing_arg, expected_error};

    use super::*;

    fn targets() -> Vec<String> {
        vec!["jon".into()]
    }

    #[test]
    /// Basic user_get test
    fn test_user_get() -> anyhow::Result<()> {
        let want = UserGetRequest { names: targets() };
        let args = UserArgs {
            action: CrudAction::Get,
            targets: targets(),
            names: vec![],
        };
        let got = user_get(args)?;

        assert_eq!(dbg!(want), dbg!(got));
        Ok(())
    }

    #[test]
    /// user_get should only accept get crud actions
    fn test_action_error() {
        let want = ClientError::CrudInvalid(CrudAction::Get, CrudAction::List);
        let args = UserArgs {
            action: CrudAction::List,
            targets: targets(),
            names: vec![],
        };
        let got = user_get(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }

    #[test]
    /// user_get should get a some value for the target.
    /// Name args should not be considered if passed to function.
    fn test_target_error() {
        let want = ClientError::ClapError(clap_missing_arg("targets"));
        let args = UserArgs {
            action: CrudAction::Get,
            targets: vec![],
            names: targets(),
        };
        let got = user_get(args).map(expected_error);

        match got {
            Ok(e) => panic!("{e}"),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
    }
}
