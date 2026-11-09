# passmgr-rs

A lightweight, encrypted CLI password manager written in Rust.

Uses Argon2id for key derivation and AES-256-GCM for authenticated encryption.

## Installation

```bash
git clone https://github.com/me0wzz/passmgr-rs.git
cd passmgr-rs
cargo build --release
```

## Usage

```bash
# Initialize vault
passmgr-rs init

# Add an account
passmgr-rs add github

# List accounts
passmgr-rs list

# Show account details
passmgr-rs show github

# Edit an account
passmgr-rs edit github

# Remove an account
passmgr-rs remove github

# Generate a random password
passmgr-rs generate --length 32
```

## Security

- Argon2id key derivation
- AES-256-GCM authenticated encryption
- Encrypted local vault storage
- Hidden master password input

## License

MIT
