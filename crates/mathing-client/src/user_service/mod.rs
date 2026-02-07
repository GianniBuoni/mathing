use std::sync::Arc;

use tonic::transport::{Channel, Endpoint};

use crate::errors::{ClientError, ServerError};
use crate::prelude::{mathing_proto::user_service_client::UserServiceClient, *};

#[derive(Default, Debug)]
pub struct UserService {
    endpoint: Arc<str>,
}

impl UserService {
    /// Initializes UserService struct with any immutable configuration variables
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
    pub async fn handle_create(&self, args: UserArgs) -> anyhow::Result<()> {
        let req = tonic::Request::new(mathing_proto::UserCreateRequest {
            name: args.name.to_string(),
        });
        let res = self.connect().await?.user_create(req).await?;

        println!("RESPONSE: {:?}", res);

        Ok(())
    }
}
