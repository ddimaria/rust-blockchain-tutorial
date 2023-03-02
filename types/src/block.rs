//! # Blocks
//!
//! Blocks are a fundamental aspect of the Ethereum blockchain.
//! A block can consist of many transactions.
//! Each block contains a hash of the parent block, which links blocks together.
//!
//! A sample block from the chain:
//!
//! {
//!   "hash": "0x7a2b18ecb9565eaa511601130d8108886b5d9cb14c6f9662c1e661bbfc73523e",
//!   "number": "0x0",
//!   "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
//!   "stateRoot": "0xd5b2d8fdfe99430dcdaa397d252d0cae3a1457c414999fbba318ba90ec0ed56b",
//!   "transactions": [],
//!   "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//! }
//!
//! see https://ethereum.org/en/developers/docs/blocks/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::{H256, U64};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Deref;
use utils::crypto::hash;

use crate::error::{Result, TypeError};
use crate::helpers::hex_to_u64;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename = "block_number")]
pub struct BlockNumber(pub U64);

/// Easily access the inner U64 of BlockNumber.
/// '''rust
/// let block_number: U64 = *BlockNumber(0);
/// '''
impl Deref for BlockNumber {
    type Target = U64;

    fn deref(&self) -> &U64 {
        &self.0
    }
}

impl From<i32> for BlockNumber {
    fn from(value: i32) -> BlockNumber {
        let parsed = U64::from(value);
        BlockNumber(parsed)
    }
}

// TODO(ddimaria): replace the custom code below with serde_with's hex macros
impl TryFrom<&str> for BlockNumber {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self> {
        let parsed = hex_to_u64(value.to_string())?;
        Ok(BlockNumber(parsed))
    }
}

// TODO(ddimaria): add in `author` once we're injecting keys into nodes
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Block {
    pub number: U64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub transactions: Vec<Transaction>,
    pub transactions_root: H256,
    pub state_root: H256,
}

impl Block {
    pub fn new(
        number: U64,
        parent_hash: H256,
        transactions: Vec<Transaction>,
        state_root: H256,
    ) -> Result<Block> {
        let transactions_root = Transaction::root_hash(&transactions)?;
        let mut block = Block {
            number,
            hash: None,
            parent_hash,
            transactions,
            transactions_root,
            state_root,
        };

        let serialized = bincode::serialize(&block)?;
        let hash: H256 = hash(&serialized).into();
        block.hash = Some(hash);

        Ok(block)
    }

    pub fn block_hash(&self) -> Result<H256> {
        self.hash.ok_or(TypeError::MissingBlockHash)
    }

    // TODO(ddimaria): add initial accounts and seed with coin (requires recalculation of the state_root)
    pub fn genesis() -> Result<Self> {
        Self::new(U64::zero(), H256::zero(), vec![], H256::zero())
    }
}
