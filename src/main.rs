use std::{error::Error, fs::File, io::Read, path::PathBuf};

use arboard::Clipboard;
use crossterm::event::{self, Event::Key, KeyCode, KeyEvent};
use rsd_encrypt::legacy_decrypt;
mod account;
mod state;
mod ui;

pub type AnyResult<T> = Result<T, Box<dyn Error>>;

use crate::{
    account::{Account, JsonAccount},
    state::{AccountCommands, AppState, UserMessage},
};

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

fn handle_input(state: AppState, key_event: KeyEvent) -> AppState {
    match state {
        AppState::Login { mut psd, message } => match key_event.code {
            KeyCode::Enter => match load_accounts(psd.clone()) {
                Ok(accounts) => match Clipboard::new() {
                    Ok(clipboard) => AppState::MainScreen {
                        clipboard,
                        accounts,
                        hovering: 0,
                        selected_command: None,
                        message: None,
                    },
                    Err(err) => {
                        psd.clear();
                        AppState::Login {
                            psd,
                            message: Some(UserMessage::Error(format!(
                                "Clipboard initialization failed: {}",
                                err
                            ))),
                        }
                    }
                },
                Err(err) => {
                    psd.clear();
                    AppState::Login {
                        psd,
                        message: Some(UserMessage::Error(format!(
                            "Password likely incorrect, error: {}",
                            err
                        ))),
                    }
                }
            },
            KeyCode::Backspace => {
                let _ = psd.pop();
                AppState::Login { psd, message }
            }
            KeyCode::Char(c) => {
                psd.push(c);
                AppState::Login { psd, message }
            }
            KeyCode::Esc => AppState::Exit,
            _ => AppState::Login { psd, message },
        },
        AppState::MainScreen {
            clipboard,
            accounts,
            hovering,
            selected_command: None,
            message: _message,
        } => match key_event.code {
            KeyCode::Enter => AppState::MainScreen {
                clipboard,
                accounts,
                selected_command: Some(AccountCommands::CopyPass),
                hovering,
                message: None,
            },
            KeyCode::Up => {
                let new_hovering = if hovering == 0 {
                    accounts.len() - 1
                } else {
                    hovering - 1
                };

                AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering: new_hovering,
                    selected_command: None,
                    message: None,
                }
            }
            KeyCode::Down => {
                let new_hovering = (hovering + 1) % accounts.len();
                AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering: new_hovering,
                    selected_command: None,
                    message: None,
                }
            }
            KeyCode::Char('c') => AppState::MainScreen {
                clipboard: accounts[hovering].copy_pass(clipboard),
                accounts,
                hovering,
                selected_command: None,
                message: None,
            },
            KeyCode::Esc => AppState::Exit,
            _ => AppState::MainScreen {
                clipboard,
                accounts,
                hovering,
                selected_command: None,
                message: None,
            },
        },
        AppState::MainScreen {
            clipboard,
            accounts,
            selected_command: Some(command),
            hovering,
            message: _message,
        } => match key_event.code {
            KeyCode::Enter => match command {
                AccountCommands::CopyPass => AppState::MainScreen {
                    clipboard: accounts[hovering].copy_pass(clipboard),
                    accounts,
                    selected_command: Some(command),
                    hovering,
                    message: Some(UserMessage::Info("Copied password to clipboard!".into())),
                },
                AccountCommands::Edit => AppState::MainScreen {
                    clipboard,
                    accounts,
                    selected_command: Some(command),
                    hovering,
                    message: Some(UserMessage::Error("Edit is not implemented yet!".into())),
                },
                AccountCommands::Delete => AppState::MainScreen {
                    clipboard,
                    accounts,
                    selected_command: Some(command),
                    hovering,
                    message: Some(UserMessage::Error("Delete is not implemented yet!".into())),
                },
            },
            KeyCode::Up => match command {
                AccountCommands::CopyPass => AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering,
                    selected_command: Some(AccountCommands::Delete),
                    message: None,
                },
                AccountCommands::Edit => AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering,
                    selected_command: Some(AccountCommands::CopyPass),
                    message: None,
                },
                AccountCommands::Delete => AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering,
                    selected_command: Some(AccountCommands::Edit),
                    message: None,
                },
            },
            KeyCode::Down => match command {
                AccountCommands::CopyPass => AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering,
                    selected_command: Some(AccountCommands::Edit),
                    message: None,
                },
                AccountCommands::Edit => AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering,
                    selected_command: Some(AccountCommands::Delete),
                    message: None,
                },
                AccountCommands::Delete => AppState::MainScreen {
                    clipboard,
                    accounts,
                    hovering,
                    selected_command: Some(AccountCommands::CopyPass),
                    message: None,
                },
            },
            KeyCode::Esc => AppState::MainScreen {
                clipboard,
                accounts,
                hovering,
                selected_command: None,
                message: None,
            },
            _ => AppState::MainScreen {
                clipboard,
                accounts,
                hovering,
                selected_command: Some(command),
                message: None,
            },
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
