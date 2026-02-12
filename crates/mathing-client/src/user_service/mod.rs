use std::sync::Arc;

use tonic::transport::{Channel, Endpoint};

use crate::errors::{ClientError, ServerError};
use crate::prelude::{mathing_proto::user_service_client::UserServiceClient, *};

mod user_create;
mod user_delete;
mod user_get;
mod user_list;

#[derive(Default, Debug)]
pub struct UserService {
    endpoint: Arc<str>,
}

impl UserService {
    /// Initializes a new UserService struct with any immutable configuration variables
    pub fn new() -> anyhow::Result<Self, ClientError> {
        let addr = std::env::var("SERVER_URI").map_err(|_| ClientError::EndpointMissing)?;

        Ok(Self {
            endpoint: format!("http://{addr}").into(),
        })
    }
    async fn connect(&self) -> anyhow::Result<UserServiceClient<Channel>> {
        let endpoint = TryInto::<Endpoint>::try_into(self.endpoint.to_string())
            .map_err(|_| ClientError::EndpointInvalid(self.endpoint.clone()))?;

        UserServiceClient::connect(endpoint)
            .await
            .map_err(|_| ServerError::Connection(self.endpoint.clone()).into())
    }
    pub async fn handle_command(&self, args: UserArgs) -> anyhow::Result<()> {
        match args.action {
            CrudAction::Create => self.handle_create(args).await?,
            CrudAction::Delete => self.handle_delete(args).await?,
            CrudAction::Get => self.handle_get(args).await?,
            CrudAction::List => self.handle_list().await?,
        }
        Ok(())
    }
}
