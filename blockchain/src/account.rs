use async_jsonrpc_client::Params;
use ethereum_types::{Address, U256};
use serde_json::Value;

use crate::block::BlockNumber;
use crate::error::Result;
use crate::helpers::to_hex;
use crate::request::send;

pub type Account = Address;

pub async fn get_all_accounts() -> Result<Vec<Account>> {
    let response = send("eth_accounts", None).await?;
    let accounts: Vec<Account> = serde_json::from_value(response)?;

    Ok(accounts)
}

pub async fn get_balance(address: Account) -> Result<U256> {
    let balance: U256 = get_balance_by_block(address, None).await?;

    Ok(balance)
}

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
    let response = send("eth_getBalance", Some(params)).await?;
    let balance: U256 = serde_json::from_value(response)?;

    Ok(balance)
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_first_user() -> Account {
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
