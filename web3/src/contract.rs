//! # Contracts
//!
//! Deploy and interact with contracts on Ethereum.

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::Address;
use ethereum_types::{H256, U256};
use types::bytes::Bytes;
use types::transaction::TransactionRequest;

use crate::error::Result;
use crate::transaction::send;

/// Deploy a contract to the chain.
///
/// # Examples
///
/// ```ignore
/// use web3::account::get_all_accounts;
/// use web3::contract::deploy;
///
/// let account = get_all_accounts().await.unwrap()[0];
/// let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
/// let tx_hash = deploy(account, &contract).await;
/// ```
pub async fn deploy(owner: Address, abi: &[u8]) -> Result<H256> {
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

    send(transaction_request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::get_all_accounts;
    use crate::helpers::tests::get_contract;

    #[tokio::test]
    async fn it_deploys_a_contract() {
        let to = get_all_accounts().await.unwrap()[1].clone();
        let data = get_contract();
        let response = deploy(to, &data).await;
        assert!(response.is_ok());
    }
}
