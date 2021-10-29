use std::env;

use actix_web::Result;
use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use crate::errors::ServiceError;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{rand_core::OsRng, SaltString}};

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

    match hashed_password {
        Err(err) => Err(err),
        Ok(data) => Ok(HashedPasswordAndSalt {
            hashed_password: data.to_string(),
            salt: salt.as_str().to_string(),
        })
    }
}

pub fn verify_password(password: &str, hash_str: &str) -> Result<(), ServiceError> {
    let hash = PasswordHash::new(hash_str);
    let argon2 = Argon2::default();
    argon2.verify_password(password.as_bytes(), &hash.unwrap()).map_err(
        |_err| {
            ServiceError::InternalServerError
        }
    )
}

#[derive(Serialize)]
pub struct JWTClaim {
    // issued at
    pub iat: i64,
    // expiration time
    pub exp: i64,
    pub email: String,
}

pub fn generate_jwt(email: &String) -> Result<String, ServiceError> {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let jwt_secret_bytes = jwt_secret.as_bytes();
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // convert nano second to second
    let one_week_in_second = 60 * 60 * 24 * 7;

    let jwt_claim = JWTClaim {
        iat: now,
        exp: now + one_week_in_second,
        email: email.to_string(),
    };
    
    let jwt_token = jsonwebtoken::encode::<JWTClaim>(&Header::default(), &jwt_claim, &EncodingKey::from_secret(jwt_secret_bytes));

    match jwt_token {
        Ok(token) => Ok(token),
        Err(_) => Err(ServiceError::InternalServerError)
    }
}