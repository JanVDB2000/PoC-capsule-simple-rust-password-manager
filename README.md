# Capsule Simple Rust Password Manager
Capsule is a minimal and secure password manager written in Rust. It uses AES-256-GCM encryption and Argon2 key derivation to securely store and manage your passwords locally.

# Features
- Master password protected vault encrypted with AES-256-GCM
- Argon2 for password-based key derivation
- Add, list, get, and search password entries
- Generate strong random passwords
- Secure password input (no echo)
- Simple CLI interface

# Installation
Ensure you have Rust and Cargo installed. Then clone and build:
```bash
git clone https://github.com/JanVDB2000/PoC-capsule-simple-rust-password-manager.git
cd capsule
cargo build --release
```
Usage Initialize a new vault:
```bash
cargo run -- init
```
Add a new entry (manual password):
```bash
cargo run -- add
```
Add a new entry with a generated password (default length 16):
```bash
cargo run -- add --generate
```

Add a new entry with a generated password of custom length:
```bash
cargo run -- add --generate --length 24
```
List all entries:
```bash
cargo run -- list
```
Get credentials for an entry:
```bash
cargo run -- get <entry_name>
```
Search entries by name:
```bash
cargo run -- search <query>
```

# Security Notes
- Your vault is stored encrypted locally at capsule.vault.
- The Argon2 salt is stored in capsule.salt.
- Master password is requested on each command.
- Password inputs are hidden for security.
- Make sure to backup your vault and salt files safely.

# Dependencies
- aes-gcm for encryption
- argon2 for key derivation
- rand for randomness
- clap for CLI parsing
- rpassword for secure password input
- serde & serde_json for serialization