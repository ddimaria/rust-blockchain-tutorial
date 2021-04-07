use async_jsonrpc_client::{HttpClient, Output, Params, Transport};
use log::*;
use serde_json::Value;

use crate::error::BlockChainError;

fn get_client() -> Result<HttpClient, BlockChainError> {
    HttpClient::new("http://127.0.0.1:8545")
        .map_err(|e| BlockChainError::ClientError(e.to_string()))
}

pub async fn send(method: &str, params: Option<Params>) -> Result<Value, BlockChainError> {
    debug!("Sending {} with params {:?}", method, params);

    let client = get_client()?;
    let response = client
        .request(method, params)
        .await
        .map_err(|e| BlockChainError::RequestError(e.to_string()))?;

    match response {
        Output::Success(s) => Ok(s.result),
        Output::Failure(f) => Err(BlockChainError::RequestError(f.error.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_sends() {
        let response = send("eth_blockNumber", None).await;
        assert!(response.is_ok());
    }
}
