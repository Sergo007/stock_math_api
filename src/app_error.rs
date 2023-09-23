// use derive_more::Display;

use std::io::ErrorKind;

#[derive(Debug)]
pub struct AppError {
    message: String,
}

// region AppError implementation
impl AppError {
    pub fn new<T>(message: T) -> Self
    where
        T: ToString,
    {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::new(value.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: value.to_string(),
        }
    }
}

// impl From<scan_fmt::parse::ScanError> for Error {
//     fn from(value: scan_fmt::parse::ScanError) -> Self {
//         Error::new(value.to_string())
//     }
// }

impl From<std::str::Utf8Error> for AppError {
    fn from(value: std::str::Utf8Error) -> Self {
        AppError::new(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::new(value.to_string())
    }
}

// impl From<uuid::Error> for Error {
//     fn from(value: uuid::Error) -> Self {
//         Error::new(value.to_string())
//     }
// }

impl From<url::ParseError> for AppError {
    fn from(value: url::ParseError) -> Self {
        AppError::new(value.to_string())
    }
}

impl From<String> for AppError {
    fn from(value: String) -> Self {
        AppError::new(value)
    }
}

// endregion
