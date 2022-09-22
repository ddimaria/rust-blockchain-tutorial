//! # Chain
//!
//! The container for the blockchain

////////////////////////////////////////////////////////////////////////////////

use crate::account::AccountData;
use crate::block::Block;
use crate::transaction::Transaction;
use blake2::{Blake2s256, Digest};
use dashmap::DashMap;
use ethereum_types::H256;
use types::account::Account;
use types::transaction::{SimpleTransaction, TransactionRequest};

#[derive(Debug)]
pub(crate) struct BlockChain {
    pub(crate) accounts: DashMap<Account, AccountData>,
    pub(crate) blocks: Vec<Block>,
    pub(crate) mempool: Vec<Transaction>,
}

impl BlockChain {
    pub(crate) fn new() -> Self {
        Self {
            accounts: DashMap::new(),
            blocks: vec![Block::genesis()],
            mempool: vec![],
        }
    }

    pub(crate) fn get_current_block(&self) -> &Block {
        self.blocks.last().unwrap().to_owned()
    }

    pub(crate) fn new_block(&mut self, transactions: Vec<SimpleTransaction>) -> H256 {
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

    pub(crate) fn send_transaction(&self, transaction_request: &TransactionRequest) -> H256 {
        // TODO: add to mempool instead
        let transaction: Transaction = transaction_request.into();

        transaction.hash.unwrap()
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
