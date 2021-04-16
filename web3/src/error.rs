//! # Errors
//!
//! Custom errors for the whole library.
//! Utility types related to errors (Result).
//! Convert errors from dependencies.

////////////////////////////////////////////////////////////////////////////////

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Web3Error {
    #[error("Error creating a new HTTP JSON-RPC client: {0}")]
    ClientError(String),

    #[error("Error serializing or deserializing JSON data: {0}")]
    JsonParseError(String),

    #[error("Error sending a HTTP JSON-RPC call: {0}")]
    RpcRequestError(String),

    #[error("Error receiving a HTTP JSON-RPC response: {0}")]
    RpcResponseError(String),
}

/// Utility result type to be used throughout
pub type Result<T> = std::result::Result<T, Web3Error>;

/// Generically convert serde errors to Web3Error::JsonParseError
impl From<serde_json::Error> for Web3Error {
    fn from(error: serde_json::Error) -> Self {
        Web3Error::JsonParseError(error.to_string())
    }
}
