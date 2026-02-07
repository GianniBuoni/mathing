use clap::Parser;

use mathing_client::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.service {
        Services::User(args) => {
            let user_service = UserService::new()?;
            user_service.handle_command(args).await?;
        }
    }

    Ok(())
}
