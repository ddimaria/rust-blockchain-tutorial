//! # Chain
//!
//! The container for the blockchain

////////////////////////////////////////////////////////////////////////////////

use std::collections::VecDeque;
use std::sync::Arc;

use crate::account::{AccountData, AccountStorage};
use crate::block::Block;
use crate::server::Context;
use crate::transaction::{Transaction, TransactionStorage};
use blake2::{Blake2s256, Digest};
use dashmap::DashMap;
use ethereum_types::H256;
use types::account::Account;
use types::transaction::{SimpleTransaction, TransactionRequest};

#[derive(Debug)]
pub(crate) struct BlockChain {
    pub(crate) accounts: AccountStorage,
    pub(crate) blocks: Vec<Block>,
    pub(crate) mempool: TransactionStorage,
}

impl BlockChain {
    pub(crate) fn new() -> Self {
        Self {
            accounts: AccountStorage::new(),
            blocks: vec![Block::genesis()],
            mempool: TransactionStorage::new(),
        }
    }

    pub(crate) fn get_current_block(&self) -> &Block {
        self.blocks.last().unwrap().to_owned()
    }

    pub(crate) fn new_block(&mut self, transactions: Vec<SimpleTransaction>) -> H256 {
        // TODO(ddimaria): make this an atomic operation
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn creates_a_blockchain() {
        let blockchain = BlockChain::new();
    }
}
