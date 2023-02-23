//! # Chain
//!
//! The container for the blockchain

////////////////////////////////////////////////////////////////////////////////

use std::collections::VecDeque;
use std::sync::Arc;

use crate::account::{AccountData, AccountStorage};
use crate::error::{ChainError, Result};
use crate::storage::Storage;
use crate::transaction::TransactionStorage;
use blake2::{Blake2s256, Digest};
use ethereum_types::{H160, H256, U256};
use tokio::sync::Mutex;
use types::block::{Block, BlockNumber};
use types::bytes::Bytes;
use types::transaction::{Transaction, TransactionReceipt, TransactionRequest};
use utils::crypto::hash;

#[derive(Debug)]
pub(crate) struct BlockChain {
    pub(crate) accounts: AccountStorage,
    pub(crate) blocks: Vec<Block>,
    pub(crate) transactions: Arc<Mutex<TransactionStorage>>,
}

impl BlockChain {
    pub(crate) fn new(storage: Arc<Storage>) -> Result<Self> {
        Ok(Self {
            accounts: AccountStorage::new(storage),
            blocks: vec![Block::genesis().unwrap()],
            transactions: Arc::new(Mutex::new(TransactionStorage::new())),
        })
    }

    pub(crate) fn get_current_block(&self) -> Result<Block> {
        let block = self
            .blocks
            .last()
            .ok_or_else(|| ChainError::BlockNotFound("current block".into()))?;

        Ok(block.to_owned())
    }

    pub(crate) fn get_block_number(&self, block_number: &str) -> Result<BlockNumber> {
        if block_number == String::from("latest") {
            Ok(BlockNumber(self.get_current_block()?.number))
        } else {
            Ok(block_number
                .try_into()
                .map_err(|_| ChainError::InvalidBlockNumber(block_number.into()))?)
        }
    }

    pub(crate) fn new_block(&mut self, transactions: Vec<Transaction>) -> Result<BlockNumber> {
        // TODO(ddimaria): make this an atomic operation
        // TODO(ddimaria): handle unwraps
        let current_block = self.get_current_block()?;
        let number = current_block.number + 1_u64;
        let parent_hash = current_block.hash.unwrap();
        let nonce_serialized = format!("{:?}", (number, parent_hash, &transactions));
        let nonce = Blake2s256::digest(&nonce_serialized);

        let block = Block::new(
            number,
            H256::from(nonce.as_ref()),
            parent_hash,
            transactions,
        )
        .unwrap();

        let hash = block.hash.unwrap();
        self.blocks.push(block);

        Ok(BlockNumber(number))
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
        )
        .unwrap();

        // TODO(ddimaria): handle unwraps
        let hash = transaction.hash.unwrap();

        // add to the transaction mempool
        self.transactions.lock().await.send_transaction(transaction);

        hash
    }

    pub(crate) async fn send_raw_transaction(&mut self, transaction: Bytes) /*-> H256*/
    {
        // let from = transaction_request.from.unwrap_or(H160::zero());
        // let nonce = self.accounts.increment_nonce(&from).unwrap().into();

        // let transaction: Transaction = Transaction::new(
        //     from,
        //     transaction_request.to.unwrap_or(H160::zero()),
        //     transaction_request.value.unwrap_or(U256::zero()),
        //     nonce,
        //     transaction_request.data,
        // );

        // // TODO(ddimaria): handle unwraps
        // let hash = transaction.hash.unwrap();

        // // add to the transaction mempool
        // self.transactions.lock().await.send_transaction(transaction);

        // hash
    }

    // TODO(ddimaria): actually process the transaction
    pub(crate) async fn process_transactions(&mut self) {
        loop {
            // Bulk drain the current queue to fit into the new block
            // This is not safe as we lose transactions if a panic occurs
            // or if the program is halted
            let transactions = self
                .transactions
                .lock()
                .await
                .mempool
                .drain(0..)
                .collect::<VecDeque<_>>();
            let mut receipts: Vec<TransactionReceipt> = vec![];

            for transaction in transactions.iter() {
                tracing::info!("Processing Transaction {:?}", transaction);

                let contract_address = transaction.data.as_ref().and_then(|_| {
                    // TODO(ddimaria): create deterministic contract address below
                    // let sender = self.accounts.get_account(&transaction.from).unwrap();
                    // let contract_address = format!("{}{}", transaction.from, sender.nonce);
                    // let contract_hash = hash(&contract_address);

                    // create a contract account
                    let account_data = AccountData::new(transaction.data.clone());
                    if let Ok(contract_address) = self.accounts.add_account(None, &account_data) {
                        Some(contract_address)
                    } else {
                        tracing::error!("Error creating a contract account {:?}", account_data);
                        None
                    }
                });

                let transaction_hash = transaction.hash.unwrap();
                let transaction_receipt = TransactionReceipt {
                    block_hash: None,
                    block_number: None,
                    contract_address,
                    transaction_hash,
                };
                receipts.push(transaction_receipt);

                self.transactions
                    .clone()
                    .lock()
                    .await
                    .processed
                    .insert(transaction_hash, transaction.to_owned());

                let transactions = self.transactions.lock().await;
                tracing::info!(
                    "Transaction storage: mempool {:?}, processed {:?}, receipts {:?}",
                    transactions.mempool.len(),
                    transactions.processed.len(),
                    transactions.receipts.len()
                );
            }

            // We've processed all transactions, now calculate the block hash
            // from the transaction hashes.  This is a naive approach.  The
            // transactions should be the leaves of a merkle tree and the block
            // hash should be the merkle root.
            let hashes = receipts
                .iter()
                .map(|transaction| transaction.transaction_hash.to_string())
                .reduce(|cur: String, nxt: String| cur + &nxt)
                .unwrap_or("".into());
            let block_hash = hash(&hashes.into_bytes());
            let block_number = self.new_block(transactions.into()).unwrap();

            tracing::info!("Created new block {}", block_number.0);

            // now add the block number and hash to the receipts
            for receipt in receipts.iter_mut() {
                receipt.block_number = Some(block_number.to_owned());
                receipt.block_hash = Some(block_hash.into());

                self.transactions
                    .clone()
                    .lock()
                    .await
                    .receipts
                    .insert(receipt.transaction_hash, receipt.to_owned());
            }
        }
    }

    pub(crate) async fn get_transaction_receipt(
        &mut self,
        transaction_hash: H256,
    ) -> Result<TransactionReceipt> {
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
    use crate::helpers::tests::STORAGE;

    #[tokio::test]
    async fn creates_a_blockchain() {
        let blockchain = BlockChain::new((*STORAGE).clone());
    }
}
