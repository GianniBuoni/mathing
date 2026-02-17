use crate::{errors, prelude::mathing_proto::UserGetRequest};

use super::*;

impl UserService {
    pub(super) async fn handle_get(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = user_get(args)?;
        let user = self
            .connect()
            .await?
            .user_get(req)
            .await?
            .into_inner()
            .user
            .ok_or(ServerError::NoneValue("UserGetResponse"))
            .map(Into::<tabled::Table>::into)?;

        println!("{user}");

        Ok(())
    }
}

fn user_get(args: UserArgs) -> Result<UserGetRequest, ClientError> {
    if args.action != CrudAction::Get {
        return Err(ClientError::CrudInvalid(CrudAction::Get, args.action));
    }
    let name = args
        .target
        .ok_or_else(|| errors::clap_missing_arg("target"))?
        .to_string();

    Ok(UserGetRequest { name })
}

#[cfg(test)]
mod tests {
    use crate::errors::{clap_missing_arg, expected_error};

    use super::*;

    const TARGET: &str = "jon";

    #[test]
    /// Basic user_get test
    fn test_user_get() -> anyhow::Result<()> {
        let want = UserGetRequest {
            name: TARGET.into(),
        };
        let args = UserArgs {
            action: CrudAction::Get,
            target: Some(TARGET.into()),
            name: None,
        };
        let got = user_get(args)?;

        assert_eq!(dbg!(want), dbg!(got));
        Ok(())
    }

    #[test]
    /// user_get should only accept get crud actions
    fn test_action_error() -> anyhow::Result<()> {
        let want = ClientError::CrudInvalid(CrudAction::Get, CrudAction::List);
        let args = UserArgs {
            action: CrudAction::List,
            target: Some(TARGET.into()),
            name: None,
        };
        let got = user_get(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }

    #[test]
    /// user_get should get a some value for the target.
    /// Name args should not be considered if passed to function.
    fn test_target_error() -> anyhow::Result<()> {
        let want = ClientError::ClapError(clap_missing_arg("target"));
        let args = UserArgs {
            action: CrudAction::Get,
            target: None,
            name: Some(TARGET.into()),
        };
        let got = user_get(args).map(expected_error);

        match got {
            Ok(e) => return Err(e),
            Err(e) => assert_eq!(want.to_string(), e.to_string()),
        }
        Ok(())
    }
}
