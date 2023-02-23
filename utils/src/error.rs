//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////

use thiserror::Error;

#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("Conversion error: {0}")]
    ConversionError(String),

    #[error("Error recovering key: {0}")]
    RecoverError(String),

    #[error("Error verifying signature: {0}")]
    VerifyError(String),
}

/// Utility result type to be used throughout
pub type Result<T> = std::result::Result<T, UtilsError>;
