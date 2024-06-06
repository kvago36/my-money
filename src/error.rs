use std::io::{self};
use std::num;

#[derive(Debug)]
pub struct AppError {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(error: serde_json::error::Error) -> Self {
        AppError {
            kind: String::from("serde"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseFloatError> for AppError {
    fn from(error: num::ParseFloatError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

// example
impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}
