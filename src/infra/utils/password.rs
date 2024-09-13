use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub fn hash_password(password: &[u8]) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let hashed_pass = Argon2::default()
        .hash_password(password, &salt)?
        .to_string();

    Ok(hashed_pass)
}

pub fn verify_password(hash: &str, password: &[u8]) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    Argon2::default().verify_password(password, &parsed_hash)
}
