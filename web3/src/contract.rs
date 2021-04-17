//! # Contracts
//!
//! Deploy and interact with contracts on Ethereum.
//!
//! see https://ethereum.org/en/developers/docs/smart-contracts/

////////////////////////////////////////////////////////////////////////////////

use async_jsonrpc_client::Params;
use ethereum_types::Address;
use ethereum_types::{H256, U256};
use serde_json::Value;
use types::block::BlockNumber;
use types::bytes::Bytes;
use types::helpers::to_hex;
use types::transaction::TransactionRequest;

use crate::error::Result;
use crate::Web3;

impl Web3 {
    /// Deploy a contract to the chain.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    /// let account = web3.get_all_accounts().await.unwrap()[0];
    /// let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
    /// let tx_hash = web3.deploy(account, &contract).await;
    /// assert!(tx_hash.is_ok());
    /// ```
    pub async fn deploy(&self, owner: Address, abi: &[u8]) -> Result<H256> {
        let gas = U256::from(1_000_000);
        let gas_price = U256::from(1_000_000);
        let data: Bytes = abi.into();
        let transaction_request = TransactionRequest {
            from: None,
            to: Some(owner),
            value: None,
            gas,
            gas_price,
            data: Some(data),
        };

        self.send(transaction_request).await
    }

    /// Get the contract code for an address
    ///
    /// See https://eth.wiki/json-rpc/API#eth_getCode
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    /// let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
    /// let tx_hash = web3.deploy(account, &contract).await.unwrap();
    /// let receipt = web3.transaction_receipt(tx_hash).await.unwrap();
    /// let code = web3.code(receipt.contract_address.unwrap(), None).await.unwrap();
    /// assert!(code.is_ok());
    /// ```
    pub async fn code(
        &self,
        address: Address,
        block_number: Option<BlockNumber>,
    ) -> Result<String> {
        let block_number = Web3::get_hex_blocknumber(block_number);
        let params = Params::Array(vec![
            Value::String(to_hex(address)),
            Value::String(block_number),
        ]);
        let response = self.send_rpc("eth_getCode", Some(params)).await?;
        let balance: String = serde_json::from_value(response)?;

        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::tests::{get_contract, web3};

    async fn deploy_contract() -> Result<H256> {
        let web3 = web3();
        let to = web3.get_all_accounts().await.unwrap()[1].clone();
        let data = get_contract();
        web3.deploy(to, &data).await
    }

    #[tokio::test]
    async fn it_deploys_a_contract() {
        let response = deploy_contract().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_a_contract_code() {
        let web3 = web3();
        let tx_hash = deploy_contract().await.unwrap();
        let receipt = web3.transaction_receipt(tx_hash).await.unwrap();
        let response = web3.code(receipt.contract_address.unwrap(), None).await;
        assert!(response.is_ok());
    }
}
