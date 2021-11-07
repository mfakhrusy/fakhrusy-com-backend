use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::model::response::ResponseBody;

#[derive(Debug, Display, Serialize, Deserialize, Clone, Copy)]
pub enum Error {
    #[display(fmt = "00001")]
    EmailOrPasswordMismatch,
}

pub fn error_code_to_message(code: Option<Error>) -> Option<String> {
    match code {
        None => None,
        Some(Error::EmailOrPasswordMismatch) => Some("Email or password mismatch".to_string()),
    }
}

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized(Error),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => HttpResponse::InternalServerError()
                .json("Internal Server Error, please please pleaseee try again maybe later"),
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(ResponseBody::<()>::new(
                    ServiceError::BadRequest(message.to_owned())
                        .to_string()
                        .as_str(),
                    None,
                    None,
                ))
            }
            ServiceError::Unauthorized(ref err) => {
                HttpResponse::Unauthorized().json(ResponseBody::<()>::new(
                    ServiceError::Unauthorized(err.to_owned())
                        .to_string()
                        .as_str(),
                    None,
                    Some(Error::EmailOrPasswordMismatch),
                ))
            }
        }
    }
}
