use std::{fs, io::Write, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;

use crate::{crypto, model::PasswordEntry};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Vault {
    entries: Vec<PasswordEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, entry: PasswordEntry) {
        self.entries.push(entry);
    }
    pub fn list(&self) -> &[PasswordEntry] {
        &self.entries
    }
    pub fn find(&self, name: &str) -> Option<&PasswordEntry> {
        self.entries.iter().find(|entry| entry.name == name)
    }
    pub fn find_mut(&mut self, name: &str) -> Option<&mut PasswordEntry> {
        self.entries.iter_mut().find(|entry| entry.name == name)
    }
    pub fn remove(&mut self, name: &str) -> bool {
        let old_len = self.entries.len();
        self.entries.retain(|entry| entry.name != name);
        old_len != self.entries.len()
    }

    pub fn save_encrypted(&self, path: &Path, password: &str) -> Result<()> {
        let json = serde_json::to_vec(self)?;
        let encrypted = crypto::encrypt(&json, password)?;
        let parent = path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        let mut temp = NamedTempFile::new_in(parent)?;

        temp.write_all(&encrypted)?;
        temp.as_file().sync_all()?;
        temp.persist(path)?;
        Ok(())
    }

    pub fn load_encrypted(path: &Path, password: &str) -> Result<Self> {
        let encrypted = fs::read(&path)?;
        let decrypted = crypto::decrypt(&encrypted, password)?;
        let vault = serde_json::from_slice(&decrypted)?;

        Ok(vault)
    }

    // deprecated
    /*
    pub fn save(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self).unwrap();
        fs::write(path, json).unwrap();
        Ok(())
    }
    pub fn load(path: &Path) -> Result<Self, Box<dyn Error>> {
        let json = fs::read_to_string(path).unwrap();
        let vault = serde_json::from_str(&json)?;
        Ok(vault)
    }
    */
}
