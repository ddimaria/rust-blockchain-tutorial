//! # Server
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::RpcModule;
use types::{account::Account, transaction::TransactionRequest};

use crate::{blockchain::BlockChain, error::Result};

pub(crate) fn eth_accounts(module: &mut RpcModule<BlockChain>) -> Result<()> {
    module.register_method("eth_accounts", |_, blockchain| {
        let accounts = blockchain.get_all_accounts();
        Ok(accounts)
    })?;

    Ok(())
}

pub(crate) fn eth_getBalance(module: &mut RpcModule<BlockChain>) -> Result<()> {
    module.register_method("eth_getBalance", move |params, blockchain| {
        let key = params.one::<Account>()?;
        let account = blockchain.get_account_balance(&key);
        Ok(account)
    })?;

    Ok(())
}

pub(crate) fn eth_sendTransaction(module: &mut RpcModule<BlockChain>) -> Result<()> {
    module.register_method("eth_sendTransaction", move |params, mut blockchain| {
        let transaction_request = params.parse::<TransactionRequest>()?;
        let transaction_hash = blockchain.send_transaction(&transaction_request);
        Ok(transaction_hash)
    })?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::types::EmptyParams;
    use types::account::Account;

    use super::*;
    use crate::account::AccountData;
    use crate::blockchain::BlockChain;

    #[tokio::test]
    async fn gets_all_accounts() {
        let blockchain = BlockChain::new();
        let account_data = AccountData::new("123".into());
        let id = blockchain.add_account(account_data);
        let mut module = RpcModule::new(blockchain);
        eth_accounts(&mut module).unwrap();
        let response: Vec<Account> = module
            .call("eth_accounts", EmptyParams::new())
            .await
            .unwrap();

        assert_eq!(response, vec!(id));
    }

    #[tokio::test]
    async fn gets_an_account_balance() {
        let blockchain = BlockChain::new();
        let account_data = AccountData::new("123".into());
        let tokens = account_data.tokens;
        let id = blockchain.add_account(account_data);
        let mut module = RpcModule::new(blockchain);
        eth_getBalance(&mut module).unwrap();
        let response: u64 = module.call("eth_getBalance", [id]).await.unwrap();

        assert_eq!(response, tokens);
    }
}
