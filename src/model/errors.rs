use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::model::response::ResponseBody;

#[derive(Debug, Display, Serialize, Deserialize, Clone, Copy)]
pub enum ServiceError {
    #[display(fmt = "00001")]
    EmailOrPasswordMismatch,
    #[display(fmt = "00002")]
    EmailAlreadyExists,
    #[display(fmt = "00003")]
    UserNotFound,
    #[display(fmt = "00004")]
    InvalidToken,
}

pub fn error_to_message(error: Option<ServiceError>) -> Option<String> {
    match error {
        None => None,
        Some(ServiceError::EmailOrPasswordMismatch) => {
            Some("Email or password mismatch".to_string())
        }
        Some(ServiceError::EmailAlreadyExists) => Some("Email already exists".to_string()),
        Some(ServiceError::UserNotFound) => Some("User not found".to_string()),
        Some(ServiceError::InvalidToken) => Some("Invalid token".to_string()),
    }
}

#[derive(Debug, Display)]
pub enum GlobalServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized(ServiceError),
}

impl ResponseError for GlobalServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            GlobalServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, please please pleaseee try again maybe later"),
            GlobalServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ResponseBody::<()>::new(
                    GlobalServiceError::BadRequest(message.to_owned())
                        .to_string()
                        .as_str(),
                    None,
                    None,
                ))
            }
            GlobalServiceError::Unauthorized(ref err) => {
                HttpResponse::Unauthorized().json(ResponseBody::<()>::new(
                    GlobalServiceError::Unauthorized(err.to_owned())
                        .to_string()
                        .as_str(),
                    None,
                    Some(ServiceError::EmailOrPasswordMismatch),
                ))
            }
        }
    }
}
