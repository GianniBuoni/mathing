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
    #[arg(value_delimiter = ',')]
    pub targets: Vec<String>,
    #[arg(long, short, value_delimiter = ',')]
    /// Name value to pass to the 'create' and 'update' action
    pub names: Vec<String>,
}

#[derive(Args)]
pub struct ItemArgs {
    pub action: CrudAction,
    #[arg(value_delimiter = ',')]
    pub targets: Vec<String>,
    #[arg(long, short, value_delimiter = ',')]
    pub names: Vec<String>,
    #[arg(long, short, value_delimiter = ',')]
    pub prices: Vec<f32>,
    #[arg(long, short)]
    pub limit: usize,
    #[arg(long, short)]
    pub page: usize,
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
