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
    /// Target database entry to perform action on.
    pub target: Option<Arc<str>>,
    #[arg(long, short)]
    /// Name value to pass to the 'create' and 'update' action
    pub name: Option<Arc<str>>,
}

#[derive(ValueEnum, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CrudAction {
    /// Creates a database entry for this entity; alias: 'add'
    #[value(alias = "add")]
    Create,
    /// Retrieve a single entry from the database
    Get,
    /// Retrieve all entries from the database; alias: 'ls'
    #[value(alias = "ls")]
    List,
    #[value(alias = "edit")]
    /// Update a database entry for this entity; alias: 'edit'
    Update,
    /// Deletes a database entry for this entity; alias: 'rm'
    #[value(alias = "rm")]
    Delete,
}
