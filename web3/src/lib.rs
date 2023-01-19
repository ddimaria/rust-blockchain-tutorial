//! # Web3
//!
//! This Web3 driver is a learning aid for understanding how real Web3 drivers
//! interact with Ethereum.

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::traits::ToRpcParams;
use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
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
        HttpClientBuilder::default()
            .build(url)
            .map_err(|e| Web3Error::ClientError(e.to_string()))
    }

    /// Send a RPC call with the given method and parameters.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    ///
    /// let response = web3.send_rpc("eth_blockNumber", rpc_params![]).await;
    /// assert!(response.is_ok());
    /// ```
    pub async fn send_rpc<Params>(&self, method: &str, params: Params) -> Result<Value>
    where
        Params: ToRpcParams + Send + std::fmt::Debug,
    {
        trace!("Sending RPC {} with params {:?}", method, params);

        let response = self
            .client
            .request(method, params)
            .await
            .map_err(|e| Web3Error::RpcRequestError(e.to_string()));

        trace!("RPC Response {:?}", response);

        response

        // match response {
        //     Output::Success(s) => Ok(s.result),
        //     Output::Failure(f) => Err(Web3Error::RpcResponseError(f.error.to_string())),
        // }
    }
}
