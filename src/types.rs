use std::{error::Error, fmt::Display};

use arboard::Clipboard;
use ratatui::text::Line;

#[allow(non_snake_case)] // Allowed due to the way the old files are serialized
#[derive(serde::Deserialize)]
pub struct JsonAccount {
    Name: String,
    Password: String,
}

pub struct Account {
    name: String,
    password: String,
}

impl Account {
    pub fn copy_pass(&self, mut clip: Clipboard) {
        clip.set_text(self.password.clone())
            .expect("Unable to copy value to clipboard");
    }
}

impl From<JsonAccount> for Account {
    fn from(value: JsonAccount) -> Self {
        Account {
            name: value.Name,
            password: value.Password,
        }
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
pub type AnyResult<T> = Result<T, Box<dyn Error>>;
