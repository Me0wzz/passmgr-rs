use aes_gcm::{Aes256Gcm, KeyInit, Nonce, aead::Aead};
use anyhow::{Ok, Result, anyhow, bail};
use argon2::Argon2;
use rand::{RngCore, rngs::OsRng};
use zeroize::Zeroizing;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const MAGIC: &[u8; 4] = b"PMPM";

fn derive_key(password: &str, salt: &[u8]) -> Result<Zeroizing<[u8; 32]>> {
    let mut key = Zeroizing::new([0u8; 32]);
    Argon2::default()
        .hash_password_into(password.as_bytes(), salt, &mut *key)
        .map_err(|e| anyhow!("Key derivation failed {e}"))?;
    Ok(key)
}

pub fn encrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let mut salt = [0u8; SALT_LEN];
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let key = derive_key(password, &salt).unwrap();
    let cipher = Aes256Gcm::new_from_slice(&*key).map_err(|_| anyhow!("Invaild encryption key"))?;
    let ciphertext = cipher
        .encrypt(&Nonce::from(nonce), data)
        .map_err(|_| anyhow!("Encryption Error"))?;
    let mut output = Vec::new();
    output.extend_from_slice(MAGIC);
    output.extend_from_slice(&salt);
    output.extend_from_slice(&nonce);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

pub fn decrypt(data: &[u8], password: &str) -> Result<Vec<u8>> {
    let header_len = MAGIC.len() + SALT_LEN + NONCE_LEN;
    // Header + AES-GCM Auth Tag (16byte) / MAGIC Check
    if data.len() < header_len + 16 || &data[..4] != MAGIC {
        bail!("Invalid Vault File");
    }
    let salt = &data[4..4 + SALT_LEN];
    let nonce = &data[4 + SALT_LEN..4 + SALT_LEN + NONCE_LEN];
    let ciphertext = &data[header_len..];

    let key = derive_key(password, salt)?;

    let cipher = Aes256Gcm::new_from_slice(&*key).map_err(|_| anyhow!("Invailed Key"))?;
    let nonce = Nonce::try_from(nonce).map_err(|_| anyhow!("Invalid Nonce"))?;
    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| anyhow!("Wrong Password || Corrupted Vault"))?;

    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto() {
        let original = b"Hello";
        let password = "master123";

        let encrypted = encrypt(original, password).unwrap();
        let decrypted = decrypt(&encrypted, password).unwrap();

        assert_eq!(original.to_vec(), decrypted);
    }
}
