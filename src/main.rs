mod model;
mod test;
mod vault;

use clap::{Command, Parser, Subcommand};
use model::PasswordEntry;

use crate::test::{test_storage, test_vault};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { name: String },
    List,
    Remove { name: String },
}

fn main() {
    test_storage().unwrap();
}
