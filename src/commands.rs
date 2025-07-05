use crate::crypto::{derive_key, generate_salt};
use crate::vault::{Entry, Vault};
use clap::{Parser, Subcommand};
use rand::Rng;
use rpassword::read_password;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(name = "Capsule")]
#[command(about = "A simple password manager in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add {
        #[arg(short, long)]
        generate: bool,
        #[arg(long, default_value_t = 16)]
        length: usize,
    },
    List,
    Get {
        name: String,
    },
    Search {
        query: String,
    },
}

pub fn handle(cli: Cli) {
    let salt = load_or_init_salt();

    match cli.command {
        Commands::Init => {
            println!("🔐 Initializing new vault.");
            let pw = prompt_password("Master password");
            let key = derive_key(&pw, &salt);
            let vault = Vault::new();
            vault.save(&key);
            println!("✅ Vault created.");
        }

        Commands::Add { generate, length } => {
            let pw = prompt_password("Master password");
            let key = derive_key(&pw, &salt);
            let mut vault = Vault::load(&key);

            let name = prompt("Entry name: ");
            let username = prompt("Username: ");
            let password = if generate {
                let pass = generate_password(length);
                println!("🔐 Generated password: {}", pass);
                pass
            } else {
                prompt_password("Password")
            };

            let entry = Entry {
                name,
                username,
                password,
                url: None,
                notes: None,
            };

            vault.entries.push(entry);
            vault.save(&key);
            println!("✅ Entry added.");
        }

        Commands::List => {
            let pw = prompt_password("Master password");
            let key = derive_key(&pw, &salt);
            let vault = Vault::load(&key);
            println!("📋 Stored entries:");
            for entry in vault.entries {
                println!("- {}", entry.name);
            }
        }

        Commands::Get { name } => {
            let pw = prompt_password("Master password");
            let key = derive_key(&pw, &salt);
            let vault = Vault::load(&key);

            match vault.entries.iter().find(|e| e.name == name) {
                Some(e) => {
                    println!("🔐 Username: {}", e.username);
                    println!("🔐 Password: {}", e.password);
                }
                None => println!("⚠️ Entry not found."),
            }
        }

        Commands::Search { query } => {
            let pw = prompt_password("Master password");
            let key = derive_key(&pw, &salt);
            let vault = Vault::load(&key);

            println!("🔍 Search results for '{}':", query);
            for entry in vault.entries.iter() {
                if entry.name.to_lowercase().contains(&query.to_lowercase()) {
                    println!("- {}", entry.name);
                }
            }
        }
    }
}

fn load_or_init_salt() -> Vec<u8> {
    let path = "capsule.salt";
    if Path::new(path).exists() {
        fs::read(path).unwrap()
    } else {
        let salt = generate_salt();
        fs::write(path, &salt).unwrap();
        salt
    }
}

fn prompt(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn prompt_password(prompt: &str) -> String {
    print!("{prompt}: ");
    io::stdout().flush().unwrap();
    read_password().unwrap()
}

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789!@#$%^&*()_+-=[]{}";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
