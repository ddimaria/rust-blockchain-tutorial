//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////v

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Error converting a hex to U64: {0}")]
    HexToU64Error(String),

    #[error("Error serializing or deserializing JSON data: {0}")]
    JsonParseError(String),
}

pub type Result<T> = std::result::Result<T, TypeError>;

impl From<serde_json::Error> for TypeError {
    fn from(error: serde_json::Error) -> Self {
        TypeError::JsonParseError(error.to_string())
    }
}
