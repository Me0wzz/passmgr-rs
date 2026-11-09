use std::{error::Error, path::Path};

use crate::{model::PasswordEntry, vault::Vault};

pub fn test_vault() {
    let mut vault = Vault::new();

    vault.add(PasswordEntry::new(
        "testname".to_string(),
        "testusername".to_string(),
        "testpassword".to_string(),
    ));
    vault.add(PasswordEntry::new(
        "testname2".to_string(),
        "testusername2".to_string(),
        "testpassword2".to_string(),
    ));

    for entry in vault.list() {
        println!("[{}]: {} / {}", entry.name, entry.username, entry.password);
    }
    if let Some(entry) = vault.find("testname2") {
        println!("find {}", entry.name);
    }
    vault.remove("testname");
    println!("Remaining entries: {}", vault.list().len());
}

pub fn test_storage() -> Result<(), Box<dyn Error>> {
    let path = Path::new("test-vault.json");
    let mut vault = Vault::new();
    vault.add(PasswordEntry::new(
        "testname".to_string(),
        "testusername".to_string(),
        "testpassword".to_string(),
    ));
    vault.save(path).unwrap();
    let loaded = Vault::load(path).unwrap();
    for entry in loaded.list() {
        println!("[{}]: {} / {}", entry.name, entry.username, entry.password);
    }
    Ok(())
}
