use std::process::exit;

use tonic::transport::Server;

use mathing_server::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Status> {
    logger_init();
    // init server config
    ServerConfig::try_init().await?;
    // init services
    let user_service = MathingUserService::default();
    //build server
    let addr = ServerEndpoint::try_get()?;

    info!("Attempting to serve at {addr}");
    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await
        .unwrap_or_else(|e| {
            error!("{e}");
            exit(1)
        });

    info!("Serving at {addr}!");
    Ok(())
}
