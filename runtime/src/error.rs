//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Error executing {0}")]
    ExecutionError(String),

    #[error("Wasmtime error {0}")]
    WasmtimeError(String),
}

/// Utility result type to be used throughout
pub type Result<T> = std::result::Result<T, RuntimeError>;

impl From<anyhow::Error> for RuntimeError {
    fn from(error: anyhow::Error) -> Self {
        RuntimeError::WasmtimeError(error.to_string())
    }
}
