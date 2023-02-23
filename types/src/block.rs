//! # Blocks
//!
//! Blocks are a fundamental aspect of the Ethereum blockchain.
//! A block can consist of many transactions.
//! Each block contains a hash of the parent block, which links blocks together.
//!
//! A sample block from the chain:
//!
//! {
//!   "difficulty": "0x0",
//!   "extraData": "0x",
//!   "gasLimit": "0x6691b7",
//!   "gasUsed": "0x0",
//!   "hash": "0x7a2b18ecb9565eaa511601130d8108886b5d9cb14c6f9662c1e661bbfc73523e",
//!   "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
//!   "miner": "0x0000000000000000000000000000000000000000",
//!   "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
//!   "nonce": "0x0000000000000000",
//!   "number": "0x0",
//!   "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
//!   "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//!   "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
//!   "size": "0x3e8",
//!   "stateRoot": "0xd5b2d8fdfe99430dcdaa397d252d0cae3a1457c414999fbba318ba90ec0ed56b",
//!   "timestamp": "0x60367687",
//!   "totalDifficulty": "0x0",
//!   "transactions": [],
//!   "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
//!   "uncles": []
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
// pub struct Block {
//     pub difficulty: U256,
//     // pub extra_data: Bytes,
//     pub gas_limit: U256,
//     pub gas_used: U256,
//     pub hash: H256,
//     pub logs_bloom: Option<Bloom>,
//     pub miner: Address,
//     pub mix_hash: Option<H256>,
//     pub nonce: Option<H64>,
//     pub number: U64,
//     pub parent_hash: H256,
//     pub receipts_root: H256,
//     pub seal_fields: Option<Vec<Bytes>>,
//     pub sha3_uncles: H256,
//     pub size: Option<U256>,
//     pub state_root: H256,
//     pub timestamp: U256,
//     pub total_difficulty: Option<U256>,
//     pub transactions: Vec<Transaction>,
//     pub transactions_root: H256,
//     pub uncles: Vec<H256>,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "snake_case", deserialize = "snake_case"))]
pub struct Block {
    pub nonce: H256,
    pub number: U64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        number: U64,
        nonce: H256,
        parent_hash: H256,
        transactions: Vec<Transaction>,
    ) -> Result<Block> {
        let mut block = Block {
            number,
            hash: None,
            nonce,
            parent_hash,
            transactions,
        };

        let serialized = bincode::serialize(&block).unwrap();
        let hashed: H256 = hash(&serialized).into();
        block.hash = Some(hashed);

        Ok(block)
    }

    pub fn genesis() -> Result<Self> {
        Self::new(U64::one(), H256::zero(), H256::zero(), vec![])
    }
}
