//! # Block
//!
//! The building blocks of the blockchain

////////////////////////////////////////////////////////////////////////////////

use blake2::{Blake2s256, Digest};
use ethereum_types::{H256, U64};
use proc_macros::NewType;
use serde::{Deserialize, Serialize};
use types::{block::SimpleBlock, transaction::SimpleTransaction};

use crate::error::Result;
use crate::helpers::serialize;

#[derive(Debug, Serialize, Deserialize, NewType, Clone)]
pub(crate) struct Block(SimpleBlock);

impl Block {
    pub(crate) fn genesis() -> Result<Self> {
        Self::new(U64::one(), H256::zero(), H256::zero(), vec![])
    }

    pub(crate) fn new(
        number: U64,
        nonce: H256,
        parent_hash: H256,
        transactions: Vec<SimpleTransaction>,
    ) -> Result<Block> {
        let block = Block(SimpleBlock {
            number,
            hash: None,
            nonce,
            parent_hash,
            transactions,
        });

        block.hash()
    }

    pub(crate) fn serialize(&self) -> Result<Vec<u8>> {
        serialize(&self)
    }

    pub(crate) fn hash(mut self) -> Result<Self> {
        let hash = Blake2s256::digest(&self.serialize()?);
        self.hash = Some(H256::from(hash.as_ref()));

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::blockchain::BlockChain;
    use crate::helpers::tests::STORAGE;

    use super::*;

    pub(crate) fn new_block(blockchain: &BlockChain) -> Result<Block> {
        let current_block = blockchain.get_current_block()?;
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
        let blockchain = BlockChain::new((*STORAGE).clone()).unwrap();
        let block = new_block(&blockchain);
        // println!("{:?}", block);
    }
}
