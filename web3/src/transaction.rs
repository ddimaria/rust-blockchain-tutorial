//! # Transactions
//!
//! Interact with Ethereum transactions.
//!
//! see https://ethereum.org/en/developers/docs/transactions/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::H256;
use jsonrpsee::rpc_params;
use serde_json::to_value;
use types::transaction::{TransactionReceipt, TransactionRequest};

use crate::error::Result;
use crate::Web3;

impl Web3 {
    /// Create a new message call transaction or deploy a contract.
    ///
    /// See https://eth.wiki/json-rpc/API#eth_sendTransaction
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use types::transaction::TransactionRequest;
    ///
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    /// let from = web3.get_all_accounts().await.unwrap()[0];
    /// let to = web3.get_all_accounts().await.unwrap()[1];
    /// let gas = U256::from(1_000_000);
    /// let gas_price = U256::from(1);
    /// let data = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
    /// let transaction_request = TransactionRequest {
    ///     from: None,
    ///     to: Some(to),
    ///     value: None,
    ///     gas,
    ///     gas_price,
    ///     data: Some(data.into()),
    ///     };
    /// let tx_hash = web3.send(transaction_request).await;
    /// ```
    pub async fn send(&self, transaction_request: TransactionRequest) -> Result<H256> {
        let transaction_request = to_value(&transaction_request)?;
        let params = rpc_params![transaction_request];
        let response = self.send_rpc("eth_sendTransaction", params).await?;
        let tx_hash: H256 = serde_json::from_value(response)?;

        Ok(tx_hash)
    }

    /// Retrieve a transaction receipt by transaction hash.
    ///
    /// See https://eth.wiki/json-rpc/API#eth_getTransactionReceipt
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use types::transaction::TransactionRequest;
    ///
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    /// let from = web3.get_all_accounts().await.unwrap()[0];
    /// let to = web3.get_all_accounts().await.unwrap()[1];
    /// let gas = U256::from(1_000_000);
    /// let gas_price = U256::from(1);
    /// let data = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
    /// let transaction_request = TransactionRequest {
    ///     from: None,
    ///     to: Some(to),
    ///     value: None,
    ///     gas,
    ///     gas_price,
    ///     data: Some(data.into()),
    ///     };
    /// let tx_hash = web3.send(transaction_request).await;
    /// let receipt = web3.transaction_receipt(tx_hash).await;
    /// ```
    pub async fn transaction_receipt(&self, tx_hash: H256) -> Result<TransactionReceipt> {
        let tx_hash = to_value(&tx_hash)?;
        let params = rpc_params![tx_hash];
        let response = self.send_rpc("eth_getTransactionReceipt", params).await?;
        let receipt: TransactionReceipt = serde_json::from_value(response)?;

        Ok(receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::tests::{get_contract, web3};
    use ethereum_types::U256;

    async fn send_transaction() -> Result<H256> {
        let web3 = web3();
        let to = web3.get_all_accounts().await.unwrap()[1];
        let gas = U256::from(1_000_000);
        let gas_price = U256::from(1);
        let data = get_contract();
        let transaction_request = TransactionRequest {
            from: None,
            to: Some(to),
            value: None,
            gas,
            gas_price,
            data: Some(data.into()),
        };
        web3.send(transaction_request).await
    }

    #[tokio::test]
    async fn it_sends_a_transaction() {
        let response = send_transaction().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_a_transaction_receipt() {
        let tx_hash = send_transaction().await.unwrap();
        let response = web3().transaction_receipt(tx_hash).await;
        assert!(response.is_ok());
    }
}
