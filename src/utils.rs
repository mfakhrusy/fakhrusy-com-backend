use actix_web::Result;
use crate::errors::ServiceError;
use argon2::{Argon2, PasswordHasher, password_hash::{rand_core::OsRng, SaltString}};

pub struct HashedPasswordAndSalt {
    pub hashed_password: String,
    pub salt: String,
}

pub fn hash_password(password: &str) -> Result<HashedPasswordAndSalt, ServiceError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_err| {
            ServiceError::InternalServerError
        });

    let hashed_password_salt = match hashed_password {
        Err(err) => Err(err),
        Ok(data) => Ok(HashedPasswordAndSalt {
            hashed_password: data.to_string(),
            salt: salt.as_str().to_string(),
        })
    };

    return hashed_password_salt;
}