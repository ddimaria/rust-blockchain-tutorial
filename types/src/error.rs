//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////v

use thiserror::Error;
use utils::error::UtilsError;

#[derive(Error, Debug)]
pub enum TypeError {
    #[error("Error encoding/decoding: {0}")]
    EncodingDecodingError(String),

    #[error("Error converting a hex to U64: {0}")]
    HexToU64Error(String),

    #[error("Error converting a hex to U64: {0}")]
    UtilError(String),
}

pub type Result<T> = std::result::Result<T, TypeError>;

impl From<Box<bincode::ErrorKind>> for TypeError {
    fn from(error: Box<bincode::ErrorKind>) -> Self {
        TypeError::EncodingDecodingError(error.to_string())
    }
}

impl From<UtilsError> for TypeError {
    fn from(error: UtilsError) -> Self {
        TypeError::UtilError(error.to_string())
    }
}
