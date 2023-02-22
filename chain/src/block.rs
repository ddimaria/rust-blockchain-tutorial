//! # Block
//!
//! The building blocks of the blockchain

////////////////////////////////////////////////////////////////////////////////

use types::block::Block;

#[cfg(test)]
mod tests {
    use crate::blockchain::BlockChain;
    use crate::helpers::tests::STORAGE;
    use blake2::{Blake2s256, Digest};
    use ethereum_types::H256;
    use types::{block::Block, transaction::Transaction};

    pub(crate) fn new_block(blockchain: &BlockChain) -> Block {
        let current_block = blockchain.get_current_block().unwrap();
        let number = current_block.number + 1_u64;
        let parent_hash = current_block.hash.unwrap();
        let transactions: Vec<Transaction> = vec![];
        let nonce_serialized = format!("{:?}", (number, parent_hash, &transactions));
        let nonce = Blake2s256::digest(&nonce_serialized);

        Block::new(
            number,
            H256::from(nonce.as_ref()),
            parent_hash,
            transactions,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn creates_a_block() {
        let blockchain = BlockChain::new((*STORAGE).clone()).unwrap();
        let block = new_block(&blockchain);
        // println!("{:?}", block);
    }
}
