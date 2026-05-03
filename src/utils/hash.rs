use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let decode_hash = PasswordHash::new(hash).expect("Failed to decode hashed password");
    let argon2 = Argon2::default();
    argon2
        .verify_password(password.as_bytes(), &decode_hash)
        .is_ok()
}
