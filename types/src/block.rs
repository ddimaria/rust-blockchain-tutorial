//! # Blocks
//!
//! Blocks are a fundamental aspect of the Ethereum blockchain.
//! A block can consist of many transactions.
//! Each block contains a hash of the parent block, which links blocks together.
//!
//! A sample block from the chain:
//!
//! {
//!     "difficulty": String("0x0"),
//!     "extraData": String("0x"),
//!     "gasLimit": String("0x6691b7"),
//!     "gasUsed": String("0x0"),
//!     "hash": String("0x7a2b18ecb9565eaa511601130d8108886b5d9cb14c6f9662c1e661bbfc73523e"),
//!     "logsBloom": String("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
//!     "miner": String("0x0000000000000000000000000000000000000000"),
//!     "mixHash": String("0x0000000000000000000000000000000000000000000000000000000000000000"),
//!     "nonce": String("0x0000000000000000"),
//!     "number": String("0x0"),
//!     "parentHash": String("0x0000000000000000000000000000000000000000000000000000000000000000"),
//!     "receiptsRoot": String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
//!     "sha3Uncles": String("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
//!     "size": String("0x3e8"),
//!     "stateRoot": String("0xd5b2d8fdfe99430dcdaa397d252d0cae3a1457c414999fbba318ba90ec0ed56b"),
//!     "timestamp": String("0x60367687"),
//!     "totalDifficulty": String("0x0"),
//!     "transactions": Array([]),
//!     "transactionsRoot": String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
//!     "uncles": Array([])
//! }
//!
//! see https://ethereum.org/en/developers/docs/blocks/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::{Address, Secret, U256, U64};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Deref;

use crate::error::{Result, TypeError};
use crate::helpers::hex_to_U64;
use crate::transaction::Transaction;

#[derive(Serialize, Deserialize, Debug)]
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

impl TryFrom<String> for BlockNumber {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self> {
        let parsed = hex_to_U64(value)?;
        Ok(BlockNumber(parsed))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub number: U64,
    pub hash: Secret,
    #[serde(rename = "parentHash")]
    pub parent_hash: Secret,
    #[serde(rename = "miner")]
    pub author: Address,
    #[serde(rename = "gasUsed")]
    pub gas_used: U256,
    #[serde(rename = "gasLimit")]
    pub gas_limit: U256,
    pub difficulty: U256,
    pub transactions: Vec<Transaction>,
}
