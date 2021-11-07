use serde::{Deserialize, Serialize};

use super::errors::{error_code_to_message, Error};

#[derive(Serialize, Deserialize)]
pub struct ResponseBody<T> {
    pub message: String,
    pub data: Option<T>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
}

impl<T> ResponseBody<T> {
    pub fn new(message: &str, data: Option<T>, error_code: Option<Error>) -> ResponseBody<T> {
        ResponseBody {
            message: message.to_string(),
            data,
            error_code: match error_code {
                None => None,
                Some(code) => Some(code.to_string()),
            },
            error_message: error_code_to_message(error_code),
        }
    }
}
