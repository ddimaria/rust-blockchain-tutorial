//! # Web3
//!
//! This Web3 driver is a learning aid for understanding how real Web3 drivers
//! interact with Ethereum.

////////////////////////////////////////////////////////////////////////////////

use async_jsonrpc_client::{HttpClient, Output, Params, Transport};
use log::*;
use serde_json::Value;

use crate::error::{Result, Web3Error};

pub mod account;
pub mod block;
pub mod contract;
pub mod error;
mod helpers;
pub mod transaction;

pub struct Web3 {
    client: HttpClient,
}

impl Web3 {
    pub fn new(url: &str) -> Result<Self> {
        let client = Web3::get_client(url)?;
        Ok(Self { client })
    }

    /// Create a new HTTP JSON-RPC client with given url.
    fn get_client(url: &str) -> Result<HttpClient> {
        HttpClient::new(url).map_err(|e| Web3Error::ClientError(e.to_string()))
    }

    /// Send a RPC call with the given method and parameters.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    ///
    /// let response = web3.send_rpc("eth_blockNumber", None).await;
    /// assert!(response.is_ok());
    /// ```
    pub async fn send_rpc(&self, method: &str, params: Option<Params>) -> Result<Value> {
        trace!("Sending RPC {} with params {:?}", method, params);

        let response = self
            .client
            .request(method, params)
            .await
            .map_err(|e| Web3Error::RpcRequestError(e.to_string()))?;

        trace!("RPC Response {:?}", response);

        match response {
            Output::Success(s) => Ok(s.result),
            Output::Failure(f) => Err(Web3Error::RpcResponseError(f.error.to_string())),
        }
    }
}
