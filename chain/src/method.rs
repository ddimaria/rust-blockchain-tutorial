//! # Json RPC Methods
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::H160;
use jsonrpsee::{types::error::CallError, RpcModule};
use types::{account::Account, transaction::TransactionRequest};

use crate::{block::Block, error::Result, server::Context};

pub(crate) fn eth_accounts(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_method("eth_accounts", |_, blockchain| {
        let accounts = blockchain.accounts.get_all_accounts();
        Ok(accounts)
    })?;

    Ok(())
}

pub(crate) fn eth_get_balance(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_method("eth_getBalance", move |params, blockchain| {
        let key = params.one::<Account>()?;
        let block = blockchain.get_current_block();
        let account = blockchain
            .accounts
            .get_account_balance_by_block(&key, block);
        Ok(account)
    })?;

    Ok(())
}

pub(crate) fn eth_get_balance_by_block(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_method("eth_getBalanceByBlock", move |params, blockchain| {
        let mut seq = params.sequence();
        let key = seq.next::<Account>()?;
        let block = seq.next::<Block>()?;
        let account = blockchain
            .accounts
            .get_account_balance_by_block(&key, &block);
        Ok(account)
    })?;

    Ok(())
}

pub(crate) fn eth_send_transaction(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_method("eth_sendTransaction", move |params, blockchain| {
        let transaction_request = params.parse::<TransactionRequest>()?;
        let transaction_hash = blockchain.mempool.send_transaction(&transaction_request);
        Ok(transaction_hash)
    })?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::types::EmptyParams;
    use types::account::Account;

    use super::*;
    use crate::helpers::tests::{assert_vec_eq, setup};

    #[tokio::test]
    async fn gets_all_accounts() {
        let (blockchain, id_1, id_2) = setup();
        let mut module = RpcModule::new(blockchain);
        eth_accounts(&mut module).unwrap();
        let response: Vec<Account> = module
            .call("eth_accounts", EmptyParams::new())
            .await
            .unwrap();

        assert_vec_eq(response, vec![id_1, id_2]);
    }

    #[tokio::test]
    async fn gets_an_account_balance() {
        let (blockchain, id_1, _) = setup();
        let tokens = blockchain.accounts.get_account(&id_1).unwrap().tokens;
        let mut module = RpcModule::new(blockchain);
        eth_get_balance(&mut module).unwrap();
        let response: u64 = module.call("eth_getBalance", [id_1]).await.unwrap();

        assert_eq!(response, tokens);
    }
}
