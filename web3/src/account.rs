//! # Accounts
//!
//! Generate Ethereum accounts and sign transactions and data.
//!
//! see https://ethereum.org/en/developers/docs/accounts/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::U256;
use jsonrpsee::rpc_params;
use types::account::Account;
use types::block::BlockNumber;
use types::helpers::to_hex;

use crate::error::Result;
use crate::Web3;

impl Web3 {
    /// Retrieve all list of all addresses/accounts.
    ///
    /// See https://eth.wiki/json-rpc/API#eth_accounts
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    /// let all_accounts = web3.get_all_accounts().await;
    /// assert!(all_accounts.is_ok());
    /// ```
    pub async fn get_all_accounts(&self) -> Result<Vec<Account>> {
        let response = self.send_rpc("eth_accounts", rpc_params![]).await?;
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
    /// let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
    /// let account = web3.get_all_accounts().await.unwrap()[0].clone();
    /// let balance = web3.get_balance(account).await;
    /// assert!(balance.is_ok());
    /// ```
    pub async fn get_balance(&self, address: Account) -> Result<U256> {
        let balance: U256 = self.get_balance_by_block(address, None).await?;

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
    /// let block = BlockNumber(0.into());
    /// let account = web3.get_all_accounts().await.unwrap()[0];
    /// let balance = web3.get_balance_by_block(account, Some(block)).await;
    /// assert!(balance.is_ok());
    /// ```
    pub async fn get_balance_by_block(
        &self,
        address: Account,
        block_number: Option<BlockNumber>,
    ) -> Result<U256> {
        let block_number = Web3::get_hex_blocknumber(block_number);
        let params = rpc_params![to_hex(address), block_number];
        let response = self.send_rpc("eth_getBalanceByBlock", params).await?;
        let balance: U256 = serde_json::from_value(response)?;

        Ok(balance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::tests::web3;

    pub async fn get_first_user() -> Account {
        let accounts = web3().get_all_accounts().await.unwrap();
        accounts[0].clone()
    }

    #[tokio::test]
    async fn it_gets_all_accounts() {
        let response = web3().get_all_accounts().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_a_balance() {
        let account = get_first_user().await;
        let response = web3().get_balance(account).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_a_balance_by_block() {
        let account = get_first_user().await;
        let response = web3()
            .get_balance_by_block(account, Some(BlockNumber(0.into())))
            .await;
        assert!(response.is_ok());
    }
}
