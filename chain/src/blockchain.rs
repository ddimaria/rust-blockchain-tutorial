//! # Chain
//!
//! The container for the blockchain

////////////////////////////////////////////////////////////////////////////////

use std::sync::Arc;

use crate::account::{AccountData, AccountStorage};
use crate::block::Block;
use crate::error::Result;
use crate::transaction::{Transaction, TransactionStorage};
use blake2::{Blake2s256, Digest};
use ethereum_types::{H160, H256, U256};
use tokio::sync::Mutex;
use types::transaction::{SimpleTransaction, SimpleTransactionReceipt, TransactionRequest};

#[derive(Debug)]
pub(crate) struct BlockChain {
    pub(crate) accounts: AccountStorage,
    pub(crate) blocks: Vec<Block>,
    pub(crate) transactions: Arc<Mutex<TransactionStorage>>,
}

impl BlockChain {
    pub(crate) fn new() -> Self {
        Self {
            accounts: AccountStorage::new(),
            blocks: vec![Block::genesis()],
            transactions: Arc::new(Mutex::new(TransactionStorage::new())),
        }
    }

    pub(crate) fn get_current_block(&self) -> Block {
        self.blocks.last().unwrap().to_owned()
    }

    pub(crate) fn new_block(&mut self, transactions: Vec<SimpleTransaction>) -> H256 {
        // TODO(ddimaria): make this an atomic operation
        // TODO(ddimaria): handle unwraps
        let current_block = self.get_current_block();
        let number = current_block.number + 1_u64;
        let parent_hash = current_block.hash.unwrap();
        let nonce_serialized = format!("{:?}", (number, parent_hash, &transactions));
        let nonce = Blake2s256::digest(&nonce_serialized);

        let block = Block::new(
            number,
            H256::from(nonce.as_ref()),
            parent_hash,
            transactions,
        );

        let hash = block.hash.unwrap();

        self.blocks.push(block);

        hash
    }

    pub(crate) async fn send_transaction(
        &mut self,
        transaction_request: TransactionRequest,
    ) -> H256 {
        let from = transaction_request.from.unwrap_or(H160::zero());
        let nonce = self.accounts.increment_nonce(&from).unwrap().into();

        let transaction: Transaction = Transaction::new(
            from,
            transaction_request.to.unwrap_or(H160::zero()),
            transaction_request.value.unwrap_or(U256::zero()),
            nonce,
            transaction_request.data,
        );

        // TODO(ddimaria): handle unwraps
        let hash = transaction.hash.unwrap();

        // add to the transaction mempool
        self.transactions.lock().await.send_transaction(transaction);

        hash
    }

    // TODO(ddimaria): actually process the transaction
    pub(crate) async fn process_transactions(&mut self) {
        loop {
            let transaction = self.transactions.lock().await.mempool.pop_front();
            if let Some(transaction) = transaction {
                tracing::info!("Processing Transaction {:?}", transaction);

                // if this is a contract deployment, create an account
                if transaction.data.is_some() {
                    // let sender = self.accounts.get_account(&transaction.from).unwrap();
                    // let contract_address = format!("{}{}", transaction.from, sender.nonce);
                    // let hash = Blake2s256::digest(&contract_address);

                    let account_data = AccountData::new(transaction.data.clone());
                    let account = self.accounts.add_account(None, account_data);
                }

                self.transactions
                    .clone()
                    .lock()
                    .await
                    .processed
                    .insert(transaction.hash.unwrap(), transaction);

                let transactions = self.transactions.lock().await;
                tracing::info!(
                    "Transaction storage: mempool {:?}, processed {:?}",
                    transactions.mempool.len(),
                    transactions.processed.len()
                );
            } else {
                break;
            }
        }
    }

    pub(crate) async fn get_transaction_receipt(
        &mut self,
        transaction_hash: H256,
    ) -> Result<SimpleTransactionReceipt> {
        let transaction_receipt = self
            .transactions
            .lock()
            .await
            .get_transaction_receipt(&transaction_hash)?;

        Ok(transaction_receipt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_a_blockchain() {
        let blockchain = BlockChain::new();
    }
}
