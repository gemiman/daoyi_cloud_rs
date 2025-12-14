use argon2::password_hash::phc::Salt;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::Rng;
use rand::distr::Alphanumeric;
use std::iter;

#[allow(dead_code)]
#[inline]
pub fn random_string(limit: usize) -> String {
    iter::repeat(())
        .map(|_| rand::rng().sample(Alphanumeric))
        .map(char::from)
        .take(limit)
        .collect()
}

pub fn verify_password(password: &str, password_hash: &str) -> anyhow::Result<()> {
    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    let result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("invalid password")),
    }
}

pub fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = Salt::generate(); // Note: needs the `getrandom` feature of `argon2` enabled
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    Ok(argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}
