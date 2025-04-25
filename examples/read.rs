use std::io::{stdin, stdout, Write as _};

#[cfg(feature = "keyring")]
use keyring_flows::{handlers::std::handle as handle_keyring_std, Entry};
#[cfg(feature = "command")]
use process_flows::{handlers::std::handle as handle_process_std, Command};
use secrecy::ExposeSecret;
#[cfg(feature = "command")]
use secret_flows::flows::ReadFromCommand;
#[cfg(feature = "keyring")]
use secret_flows::flows::ReadFromKeyring;
use secret_flows::Secret;

fn main() {
    env_logger::init();

    let secret = match read_line("Backend (command, keyring)?").as_str() {
        #[cfg(feature = "command")]
        "command" => {
            let args = read_line("Command?");
            let mut args = args.split_whitespace();
            let mut command = Command::new(args.next().unwrap());
            command.args(args);
            Secret::Command(command)
        }
        #[cfg(not(feature = "command"))]
        "command" => {
            panic!("missing `command` cargo feature");
        }
        #[cfg(feature = "keyring")]
        "keyring" => {
            let name = read_line("Keyring entry name?");
            let entry = Entry::new(name);
            Secret::Keyring(entry)
        }
        #[cfg(not(feature = "keyring"))]
        "keyring" => {
            panic!("missing `keyring` cargo feature");
        }
        backend => {
            panic!("unknown backend {backend}");
        }
    };

    match secret {
        Secret::Raw(_) => unreachable!(),
        #[cfg(feature = "command")]
        Secret::Command(cmd) => {
            let mut read = ReadFromCommand::new(cmd);
            loop {
                match read.next() {
                    Ok(secret) => break println!("secret: {}", secret.expose_secret()),
                    Err(io) => handle_process_std(&mut read, io).unwrap(),
                }
            }
        }
        #[cfg(feature = "keyring")]
        Secret::Keyring(entry) => {
            let mut read = ReadFromKeyring::new(entry);
            loop {
                match read.next() {
                    Ok(secret) => break println!("secret: {}", secret.expose_secret()),
                    Err(io) => handle_keyring_std(&mut read, io).unwrap(),
                }
            }
        }
    }
}

fn read_line(prompt: &str) -> String {
    print!("{prompt} ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    line.trim().to_owned()
}
