use std::env;

use mathing_server::*;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::var("SERVER_URI")?.parse()?;
    // init services
    let user_service = MathingUserService::default();
    //build server
    println!("Server starting on {}", addr);
    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
