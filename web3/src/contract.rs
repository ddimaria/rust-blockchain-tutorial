//! # Contracts
//!
//! Deploy and interact with contracts on Ethereum.
//!
//! see https://ethereum.org/en/developers/docs/smart-contracts/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::Address;
use ethereum_types::{H256, U256};
use types::bytes::Bytes;
use types::transaction::TransactionRequest;

use crate::error::Result;
use crate::Web3;

impl Web3 {
    /// Deploy a contract to the chain.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use web3::account::get_all_accounts;
    /// use web3::contract::deploy;
    ///
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
}

#[cfg(test)]
mod tests {
    use crate::helpers::tests::{get_contract, web3};

    #[tokio::test]
    async fn it_deploys_a_contract() {
        let web3 = web3();
        let to = web3.get_all_accounts().await.unwrap()[1].clone();
        let data = get_contract();
        let response = web3.deploy(to, &data).await;
        assert!(response.is_ok());
    }
}
