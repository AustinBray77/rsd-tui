use std::{fs::File, io::Read, path::PathBuf};

use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use rsd_encrypt::legacy_decrypt;
mod io;
mod types;
mod ui;
use types::AnyResult;

use crate::types::{Account, JsonAccount};

fn decrypt_contents(contents: String, key: String) -> AnyResult<String> {
    Ok(legacy_decrypt(contents, key)?)
}

fn parse_contents(contents: String) -> AnyResult<Vec<Account>> {
    let json_accounts: Vec<JsonAccount> = serde_json::from_str(&contents)?;

    Ok(json_accounts
        .into_iter()
        .map(Into::<Account>::into)
        .collect())
}

fn load_accounts(password: String) -> AnyResult<Vec<Account>> {
    let src_path: PathBuf = dirs::home_dir()
        .ok_or("Home_dir does not exist")?
        .join(".local/share/rsd-tui/psd.bin");

    let mut src_file = File::open(src_path)?;

    let mut enc_contents = String::new();

    src_file.read_to_string(&mut enc_contents)?;

    let contents = decrypt_contents(enc_contents, password)?;

    // TODO: Handle empty case better when the TUI is upgraded
    let accounts: Vec<Account> = parse_contents(contents)?;

    Ok(accounts)
}

enum AppState {
    Login {
        psd: String,
        alert: Option<String>,
    },
    MainScreen {
        accounts: Vec<Account>,
        selected: usize,
    },
    Exit,
}

impl Default for AppState {
    fn default() -> Self {
        Self::Login {
            psd: String::new(),
            alert: None,
        }
    }
}

fn handle_input(state: AppState, key_event: KeyEvent) -> AppState {
    match state {
        AppState::Login { mut psd, alert } => match key_event.code {
            KeyCode::Enter => match load_accounts(psd.clone()) {
                Ok(accounts) => AppState::MainScreen {
                    accounts,
                    selected: 0,
                },
                Err(err) => {
                    psd.clear();
                    AppState::Login {
                        psd,
                        alert: Some(err.to_string()),
                    }
                }
            },
            KeyCode::Backspace => {
                let _ = psd.pop();
                AppState::Login { psd, alert }
            }
            KeyCode::Char(c) => {
                psd.push(c);
                AppState::Login { psd, alert }
            }
            KeyCode::Esc => AppState::Exit,
            _ => AppState::Login { psd, alert },
        },
        AppState::MainScreen { accounts, selected } => match key_event.code {
            KeyCode::Enter => AppState::MainScreen { accounts, selected },
            KeyCode::Up => {
                let new_selected = if selected == 0 {
                    accounts.len() - 1
                } else {
                    selected - 1
                };

                AppState::MainScreen {
                    accounts,
                    selected: new_selected,
                }
            }
            KeyCode::Down => {
                let new_selected = (selected + 1) % accounts.len();
                AppState::MainScreen {
                    accounts,
                    selected: new_selected,
                }
            }
            KeyCode::Esc => AppState::Exit,
            _ => AppState::MainScreen { accounts, selected },
        },
        _ => state,
    }
}

fn main() -> std::io::Result<()> {
    let mut state = AppState::default();

    ratatui::run(|terminal| {
        loop {
            if let AppState::Exit = state {
                break Ok(());
            }

            terminal.draw(|frame| crate::ui::render_ui(frame, &state))?;

            state = match event::read()? {
                Key(key_event) => handle_input(state, key_event),
                _ => state,
            }
        }
    })
}

// TODO: Possibly move to better error handling
// I feel like panicing in main is probably fine tho
/*fn main() {
    let src_path: PathBuf = dirs::home_dir()
        .expect("Expected home dir to exist")
        .join(".local/share/rsd-tui/psd.bin");

    let mut src_file = File::open(src_path).expect("Expected file to exist, panicing...");

    let mut enc_contents = String::new();

    src_file
        .read_to_string(&mut enc_contents)
        .expect("Unable to read file contents, panicing...");

    let contents = decrypt_contents(enc_contents).expect("Unable to decrypt the file");

    // TODO: Handle empty case better when the TUI is upgraded
    let accounts: Box<[Account]> = parse_contents(contents)
        .expect("Unable to parse decrypted file")
        .into();

    println!("Available PSDs");

    accounts.iter().enumerate().for_each(print_account);

    println!("Which account would you like to copy to clipboard?");

    let inputted_num = read_u32(accounts.len() as u32).expect("Invalid input, panicing...");

    let account = &accounts[inputted_num as usize];

    let clipboard = arboard::Clipboard::new().expect("Unable to initialize clipboard");

    account.copy_pass(clipboard);

    println!("Copied password for {} to clipboard.", account);
}*/
