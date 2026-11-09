use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub name: String,
    pub username: String,
    pub password: String,
}

impl PasswordEntry {
    pub fn new(name: String, username: String, password: String) -> Self {
        Self {
            name,
            username,
            password,
        }
    }
}
