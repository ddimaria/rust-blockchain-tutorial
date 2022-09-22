//! # Block
//!
//! The building blocks of the blockchain

////////////////////////////////////////////////////////////////////////////////

use blake2::{Blake2s256, Digest};
use ethereum_types::{H256, H64, U64};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use types::{block::SimpleBlock, transaction::SimpleTransaction};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Block(SimpleBlock);

// Deref and DerefMut make working with newtypes easy
impl Deref for Block {
    type Target = SimpleBlock;

    fn deref(&self) -> &SimpleBlock {
        &self.0
    }
}

impl DerefMut for Block {
    fn deref_mut(&mut self) -> &mut SimpleBlock {
        &mut self.0
    }
}

impl Block {
    pub(crate) fn genesis() -> Self {
        Self::new(U64::one(), H256::zero(), H256::zero(), vec![])
    }

    pub(crate) fn new(
        number: U64,
        nonce: H256,
        parent_hash: H256,
        transactions: Vec<SimpleTransaction>,
    ) -> Block {
        let block = Block(SimpleBlock {
            number,
            hash: None,
            nonce,
            parent_hash,
            transactions,
        });
        block.hash()
    }

    pub(crate) fn serialize(&self) -> String {
        format!(
            "{:?}",
            (
                &self.nonce,
                &self.number,
                &self.parent_hash,
                &self.transactions
            )
        )
    }

    pub(crate) fn hash(mut self) -> Self {
        let hash = Blake2s256::digest(&self.serialize());
        self.hash = Some(H256::from(hash.as_ref()));
        self
    }

    pub(crate) fn is_signed(&self) -> bool {
        self.hash.is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::BlockChain;

    use super::*;

    pub(crate) fn new_block(blockchain: &BlockChain) -> Block {
        let current_block = blockchain.get_current_block();
        let number = current_block.number + 1_u64;
        let parent_hash = current_block.hash.unwrap();
        let transactions: Vec<SimpleTransaction> = vec![];
        let nonce_serialized = format!("{:?}", (number, parent_hash, &transactions));
        let nonce = Blake2s256::digest(&nonce_serialized);

        let block = Block::new(
            number,
            H256::from(nonce.as_ref()),
            parent_hash,
            transactions,
        );

        block
    }

    #[tokio::test]
    async fn creates_a_block() {
        let blockchain = BlockChain::new();
        let block = new_block(&blockchain);
        println!("{:?}", block);
    }
}
