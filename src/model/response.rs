use serde::{Deserialize, Serialize};

use super::errors::{error_to_message, ServiceError};

#[derive(Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: Option<T>, error: Option<ServiceError>) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
            error_code: match error {
                None => None,
                Some(code) => Some(code.to_string()),
            },
            error_message: error_to_message(error),
        }
    }
}
