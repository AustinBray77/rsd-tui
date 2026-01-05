use std::fmt::Display;

use arboard::Clipboard;

use crate::account::Account;

pub const COMMAND_COUNT: usize = 3;
pub const COMMAND_STRS: [&'static str; COMMAND_COUNT] =
    ["Copy Password", "Edit Account", "Delete Account"];

#[derive(Clone, Copy)]
pub enum AccountCommands {
    CopyPass = 0,
    Edit = 1,
    Delete = 2,
}

pub enum UserMessage {
    Info(String),
    Error(String),
}

pub enum AppState {
    Login {
        psd: String,
        message: Option<UserMessage>,
    },
    MainScreen {
        clipboard: Clipboard,
        accounts: Vec<Account>,
        hovering: usize,
        selected_command: Option<AccountCommands>,
        message: Option<UserMessage>,
    },
    Exit,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Login {
            psd: String::new(),
            message: None,
        }
    }
}

impl Display for AccountCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", COMMAND_STRS[*self as usize])
    }
}
