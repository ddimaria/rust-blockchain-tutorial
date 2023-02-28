//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::core::Error as JsonRpseeError;
use serde::{Deserialize, Serialize};
use std::{net::AddrParseError, sync::PoisonError};
use thiserror::Error;
use tracing_subscriber::{
    filter::{FromEnvError, ParseError as TracingParseError},
    util::TryInitError as TracingTryInitError,
};
use types::error::TypeError;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ChainError {
    #[error("Error parsing address {0}")]
    AddrParseError(String),

    #[error("Account {0} not found")]
    AccountNotFound(String),

    #[error("Block {0} not found")]
    BlockNotFound(String),

    #[error("Could not create root hash for : {0}")]
    CannotCreateRootHash(String),

    #[error("Error encoding/decoding: {0}")]
    EncodingDecodingError(String),

    #[error("Could not deserialize: {0}")]
    DeserializeError(String),

    #[error("Interal Error: {0}")]
    InteralError(String),

    #[error("Invalid block number {0}")]
    InvalidBlockNumber(String),

    #[error("JsonRpsee Error: {0}")]
    JsonRpseeError(String),

    #[error("Parent hash is missing: {0}")]
    MissingHash(String),

    #[error("Could not serialize: {0}")]
    SerializeError(String),

    #[error("Could not open the database: {0}")]
    StorageCannotOpenDb(String),

    #[error("Could not destroy the database: {0}")]
    StorageDestroyError(String),

    #[error("Could not find {0} in storage")]
    StorageNotFound(String),

    #[error("Could put {0} in storage")]
    StoragePutError(String),

    #[error("Error parsing EnvFilter from an environment variable {0}")]
    TracingFromEnvError(String),

    #[error("Error parsing {0} as a filtering directive")]
    TracingParseError(String),

    #[error("The tracing global default subscriber could not be initialized: {0}")]
    TracingTryInitError(String),

    #[error("Transaction {0} not found")]
    TransactionNotFound(String),

    #[error("Transaction {0} cannot be verified")]
    TransactionNotVerified(String),

    #[error("Type Error {0}")]
    TypeError(String),
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

impl From<ChainError> for JsonRpseeError {
    fn from(error: ChainError) -> Self {
        JsonRpseeError::Custom(error.to_string())
    }
}

impl<T> From<PoisonError<T>> for ChainError {
    fn from(error: PoisonError<T>) -> Self {
        ChainError::JsonRpseeError(error.to_string())
    }
}

impl From<TypeError> for ChainError {
    fn from(error: TypeError) -> Self {
        ChainError::TypeError(error.to_string())
    }
}
impl From<Box<bincode::ErrorKind>> for ChainError {
    fn from(error: Box<bincode::ErrorKind>) -> Self {
        ChainError::EncodingDecodingError(error.to_string())
    }
}
