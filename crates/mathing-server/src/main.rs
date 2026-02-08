use mathing_server::prelude::*;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init server config
    ServerConfig::try_init().await?;
    // init services
    let user_service = MathingUserService::default();
    //build server
    let addr = ServerEndpoint::try_get()?;
    println!("Server starting on {}", addr);
    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
