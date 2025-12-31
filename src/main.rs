use std::{fs::File, io::Read, path::PathBuf};

use arboard;
use rpassword;
use rsd_encrypt::legacy_decrypt;
mod io;
mod types;

use io::read_u32;
use types::AnyResult;

use crate::types::{Account, JsonAccount};

fn decrypt_contents(contents: String) -> AnyResult<String> {
    println!("Enter password:");

    let encryption_key = rpassword::read_password()?;

    // TODO: Modify the rsd_encrypt crate so the Error impl std::error::Error
    // Will allow me to get rid of this expect
    Ok(legacy_decrypt(contents, encryption_key).expect("Unable to decrypt the file"))
}

fn print_account((ind, account): (usize, &Account)) {
    println!("{}) {}", ind, account);
}

fn parse_contents(contents: String) -> AnyResult<Vec<Account>> {
    let json_accounts: Vec<JsonAccount> = serde_json::from_str(&contents)?;

    Ok(json_accounts
        .into_iter()
        .map(Into::<Account>::into)
        .collect())
}

// TODO: Possibly move to better error handling
// I feel like panicing in main is probably fine tho
fn main() {
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
}
