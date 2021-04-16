//! # Accounts
//!
//! Generate Ethereum accounts and sign transactions and data.

////////////////////////////////////////////////////////////////////////////////

use async_jsonrpc_client::Params;
use ethereum_types::U256;
use serde_json::Value;
use types::account::Account;
use types::block::BlockNumber;
use types::helpers::to_hex;

use crate::error::Result;
use crate::request::send_rpc;

/// Retrieve all list of all addresses/accounts.
///
/// See https://eth.wiki/json-rpc/API#eth_accounts
///
/// # Examples
///
/// ```ignore
/// use web3::account::get_all_accounts;
///
/// let all_accounts = get_all_accounts().await;
/// ```
pub async fn get_all_accounts() -> Result<Vec<Account>> {
    let response = send_rpc("eth_accounts", None).await?;
    let accounts: Vec<Account> = serde_json::from_value(response)?;

    Ok(accounts)
}

/// Retrieve the eth balance for an accout at the current block.
///
/// See https://eth.wiki/json-rpc/API#eth_getBalance
///
/// # Examples
///
/// ```ignore
/// use web3::account::{get_all_accounts, get_balance};
///
/// let account = get_all_accounts().await.unwrap()[0].clone();
/// let balance = get_balance(account).await;
/// ```
pub async fn get_balance(address: Account) -> Result<U256> {
    let balance: U256 = get_balance_by_block(address, None).await?;

    Ok(balance)
}

/// Retrieve the eth balance for an accout at a given block.
///
/// See https://eth.wiki/json-rpc/API#eth_getBalance
///
/// # Examples
///
/// ```ignore
/// use types::block::BlockNumber;
/// use web3::account::{get_all_accounts, get_balance_by_block};
///
/// let block = BlockNumber(0.into());
/// let account = get_all_accounts().await.unwrap()[0];
/// let balance = get_balance_by_block(account, Some(block)).await;
/// ```
pub async fn get_balance_by_block(
    address: Account,
    block_number: Option<BlockNumber>,
) -> Result<U256> {
    let block_number = block_number.map_or_else(
        || "latest".to_string(),
        |block_number| to_hex(*block_number),
    );
    let params = Params::Array(vec![
        Value::String(to_hex(address)),
        Value::String(block_number),
    ]);
    let response = send_rpc("eth_getBalance", Some(params)).await?;
    let balance: U256 = serde_json::from_value(response)?;

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    pub async fn get_first_user() -> Account {
        let accounts = get_all_accounts().await.unwrap();
        accounts[0].clone()
    }

    #[tokio::test]
    async fn it_gets_all_accounts() {
        let response = get_all_accounts().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_a_balance() {
        let account = get_first_user().await;
        let response = get_balance(account).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_a_balance_by_block() {
        let account = get_first_user().await;
        let response = get_balance_by_block(account, Some(BlockNumber(0.into()))).await;
        assert!(response.is_ok());
    }
}
