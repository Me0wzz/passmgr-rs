mod crypto;
mod model;
mod test;
mod vault;

use std::{
    io::{self, Read, Write},
    path::Path,
};

use anyhow::{Result, bail};
use clap::{Command, Parser, Subcommand};
use model::PasswordEntry;
use zeroize::Zeroizing;

use crate::{
    test::{test_storage, test_vault},
    vault::Vault,
};

const VAULT_PATH: &str = "vault.enc";

#[derive(Parser, Debug)]
#[command(name = "passmgr-rs")]
#[command(about = "simple password manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Add { name: String },
    List,
    Show { name: String },
    Remove { name: String },
}

fn prompt(message: &str) -> Result<String> {
    print!("{message}");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn master_password() -> Result<Zeroizing<String>> {
    let password = rpassword::prompt_password("Master password: ")?;
    Ok(Zeroizing::new(password))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = Path::new(VAULT_PATH);
    match cli.command {
        Commands::Init => {
            if path.exists() {
                bail!("Vault already exists!");
            }
            let password = master_password()?;
            if password.is_empty() {
                bail!("Master password should not be empty!");
            }
            let confirm = Zeroizing::new(rpassword::prompt_password("Confirm Password: ")?);

            if *password != *confirm {
                bail!("Password does not match!");
            }
            let vault = Vault::new();
            vault.save_encrypted(path, &password)?;
            println!("Vault initialized successfully");
        }
        Commands::Add { name } => {
            let password = master_password()?;
            let mut vault = Vault::load_encrypted(path, &password)?;

            if vault.find(&name).is_some() {
                bail!("Account already exists");
            }
            let username = prompt("Username: ")?;
            let account_password = rpassword::prompt_password("Password: ")?;
            let entry = PasswordEntry::new(name.clone(), username, account_password);
            vault.add(entry);
            vault.save_encrypted(path, &password)?;

            println!("Account '{name}' added successfully.");
        }
        Commands::List => {
            let password = master_password()?;
            let vault = Vault::load_encrypted(path, &password)?;
            for entry in vault.list() {
                println!("{} ({})", entry.name, entry.username);
            }
        }
        Commands::Show { name } => {
            let password = master_password()?;
            let vault = Vault::load_encrypted(path, &password)?;
            match vault.find(&name) {
                Some(entry) => {
                    println!("Name: {}", entry.name);
                    println!("Username: {}", entry.username);
                    println!("Password: *******");
                }
                None => println!("Account not found!"),
            }
        }
        Commands::Remove { name } => {
            let password = master_password()?;
            let mut vault = Vault::load_encrypted(path, &password)?;
            if vault.remove(&name) {
                vault.save_encrypted(path, &password)?;
                println!("Account {} removed.", name);
            } else {
                println!("Account not found.");
            }
        }
    }
    Ok(())
}
