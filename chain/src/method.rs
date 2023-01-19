//! # Json RPC Methods
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::H256;
use jsonrpsee::core::Error;
use jsonrpsee::RpcModule;
use types::{
    account::Account,
    block::BlockNumber,
    helpers::to_hex,
    transaction::{SimpleTransactionReceipt, TransactionRequest},
};

use crate::{error::Result, server::Context};

pub(crate) fn eth_accounts(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method("eth_accounts", |_, blockchain| async move {
        let accounts = blockchain.lock().await.accounts.get_all_accounts();
        Ok(accounts)
    })?;

    Ok(())
}

pub(crate) fn eth_block_number(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method("eth_blockNumber", |_, blockchain| async move {
        let block_number = blockchain.lock().await.get_current_block().number;
        Ok(block_number)
    })?;

    Ok(())
}

pub(crate) fn eth_get_block_by_number(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method("eth_getBlockByNumber", |params, blockchain| async move {
        let block_number = params.one::<BlockNumber>()?;
        let index = (*block_number).as_usize();

        Ok(blockchain.lock().await.blocks[index - 1].clone())
    })?;

    Ok(())
}

pub(crate) fn eth_get_balance(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method("eth_getBalance", move |params, blockchain| async move {
        let key = params.one::<Account>()?;
        let block = blockchain.lock().await.get_current_block().number;
        let account = blockchain
            .lock()
            .await
            .accounts
            .get_account_balance_by_block(&key, &BlockNumber(block))
            .map_err(|e| Error::Custom(e.to_string()))?;

        Ok(account)
    })?;

    Ok(())
}

pub(crate) fn eth_get_balance_by_block(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method(
        "eth_getBalanceByBlock",
        move |params, blockchain| async move {
            let mut seq = params.sequence();
            let account = seq.next::<Account>()?;
            let block = seq.next::<String>()?.clone();
            let block_number = if block == String::from("latest") {
                BlockNumber(blockchain.lock().await.get_current_block().number)
            } else {
                block
                    .try_into()
                    .map_err(|_| Error::Custom(format!("Invalid block number")))?
            };

            let balance = blockchain
                .lock()
                .await
                .accounts
                .get_account_balance_by_block(&account, &block_number)
                .map_err(|e| Error::Custom(e.to_string()))?;

            Ok(to_hex(balance))
        },
    )?;

    Ok(())
}

pub(crate) fn eth_send_transaction(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method(
        "eth_sendTransaction",
        move |params, blockchain| async move {
            let transaction_request = params.one::<TransactionRequest>()?;
            let transaction_hash = blockchain
                .lock()
                .await
                .send_transaction(transaction_request)
                .await;

            Ok(transaction_hash)
        },
    )?;

    Ok(())
}

pub(crate) fn eth_get_transaction_receipt(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method(
        "eth_getTransactionReceipt",
        move |params, blockchain| async move {
            let transaction_hash = params.one::<H256>()?;
            let transaction_receipt = blockchain
                .lock()
                .await
                .get_transaction_receipt(transaction_hash)
                .await
                .map_err(|e| Error::Custom(e.to_string()))?;

            Ok(transaction_receipt)
        },
    )?;

    Ok(())
}

pub(crate) fn eth_get_code(module: &mut RpcModule<Context>) -> Result<()> {
    module.register_async_method("eth_getCode", move |params, blockchain| async move {
        let mut seq = params.sequence();
        let address = seq.next::<Account>()?;

        // TODO(ddimaria): lookup code by block number
        // let _block = seq.next::<BlockNumber>()?;
        let block = seq.next::<String>()?.clone();
        let _block_number = if block == String::from("latest") {
            BlockNumber(blockchain.lock().await.get_current_block().number)
        } else {
            block
                .try_into()
                .map_err(|_| Error::Custom(format!("Invalid block number")))?
        };

        let code_hash = blockchain
            .lock()
            .await
            .accounts
            .get_account(&address)
            .map_err(|e| Error::Custom(e.to_string()))?
            .code_hash
            .unwrap();

        Ok(code_hash)
    })?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::rpc_params;
    use types::account::Account;

    use super::*;
    use crate::helpers::tests::{assert_vec_eq, setup};

    #[tokio::test]
    async fn gets_all_accounts() {
        let (blockchain, id_1, id_2) = setup().await;
        let mut module = RpcModule::new(blockchain);
        eth_accounts(&mut module).unwrap();
        let response: Vec<Account> = module.call("eth_accounts", rpc_params![]).await.unwrap();

        assert_vec_eq(response, vec![id_1, id_2]);
    }

    #[tokio::test]
    async fn gets_an_account_balance() {
        let (blockchain, id_1, _) = setup().await;
        let balance = blockchain
            .lock()
            .await
            .accounts
            .get_account(&id_1)
            .unwrap()
            .balance;
        let mut module = RpcModule::new(blockchain);
        eth_get_balance(&mut module).unwrap();
        let response: u64 = module.call("eth_getBalance", [id_1]).await.unwrap();

        assert_eq!(response, balance);
    }
}
