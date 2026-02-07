use std::sync::Arc;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub service: Services,
}

#[derive(Subcommand)]
pub enum Services {
    User(UserArgs),
}

#[derive(Args)]
pub struct UserArgs {
    pub action: CrudAction,
    pub name: Arc<str>,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CrudAction {
    Create,
    Delete,
}
