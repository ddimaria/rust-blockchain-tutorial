use async_jsonrpc_client::{HttpClient, Output, Params, Transport};
use log::*;
use serde_json::Value;

use crate::error::{Result, Web3Error};

fn get_client() -> Result<HttpClient> {
    HttpClient::new("http://127.0.0.1:8545").map_err(|e| Web3Error::ClientError(e.to_string()))
}

pub async fn send_rpc(method: &str, params: Option<Params>) -> Result<Value> {
    debug!("Sending {} with params {:?}", method, params);

    let client = get_client()?;
    let response = client
        .request(method, params)
        .await
        .map_err(|e| Web3Error::RequestError(e.to_string()))?;

    match response {
        Output::Success(s) => Ok(s.result),
        Output::Failure(f) => Err(Web3Error::RequestError(f.error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_sends() {
        let response = send_rpc("eth_blockNumber", None).await;
        assert!(response.is_ok());
    }
}
