//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::core::Error as JsonRpseeError;
use std::{net::AddrParseError, sync::PoisonError};
use thiserror::Error;
use tracing_subscriber::{
    filter::{FromEnvError, ParseError as TracingParseError},
    util::TryInitError as TracingTryInitError,
};

#[derive(Error, Debug)]
pub enum ChainError {
    #[error("Error parsing address {0}")]
    AddrParseError(String),

    #[error("Account {0} not found")]
    AccountNotFound(String),

    #[error("JsonRpsee Error: {0}")]
    JsonRpseeError(String),

    #[error("Could not deserialize for storage: {0}")]
    StorageDeserialize(String),

    #[error("Could not {0} in storage")]
    StorageNotFound(String),

    #[error("Could not serialize for storage: {0}")]
    StorageSerialize(String),

    #[error("Error parsing EnvFilter from an environment variable {0}")]
    TracingFromEnvError(String),

    #[error("Error parsing {0} as a filtering directive")]
    TracingParseError(String),

    #[error("The tracing global default subscriber could not be initialized: {0}")]
    TracingTryInitError(String),

    #[error("Transaction {0} not found")]
    TransactionNotFound(String),
}

/// Utility result type to be used throughout
pub type Result<T> = std::result::Result<T, ChainError>;

impl From<AddrParseError> for ChainError {
    fn from(error: AddrParseError) -> Self {
        ChainError::AddrParseError(error.to_string())
    }
}

impl From<FromEnvError> for ChainError {
    fn from(error: FromEnvError) -> Self {
        ChainError::TracingFromEnvError(error.to_string())
    }
}

impl From<TracingParseError> for ChainError {
    fn from(error: TracingParseError) -> Self {
        ChainError::TracingParseError(error.to_string())
    }
}

impl From<TracingTryInitError> for ChainError {
    fn from(error: TracingTryInitError) -> Self {
        ChainError::TracingTryInitError(error.to_string())
    }
}

impl From<JsonRpseeError> for ChainError {
    fn from(error: JsonRpseeError) -> Self {
        ChainError::JsonRpseeError(error.to_string())
    }
}

impl<T> From<PoisonError<T>> for ChainError {
    fn from(error: PoisonError<T>) -> Self {
        ChainError::JsonRpseeError(error.to_string())
    }
}
