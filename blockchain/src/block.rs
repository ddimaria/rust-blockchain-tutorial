use async_jsonrpc_client::Params;
use ethereum_types::{Address, Secret, U256, U64};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::convert::TryFrom;
use std::ops::Deref;

use crate::error::{BlockChainError, Result};
use crate::helpers::{hex_to_U64, to_hex};
use crate::request::send;
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
    type Error = BlockChainError;

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

pub async fn get_block_number() -> Result<BlockNumber> {
    let response = send("eth_blockNumber", None).await?;
    let block_number: BlockNumber = serde_json::from_value(response)?;

    Ok(block_number)
}

pub async fn get_block(block_number: U64) -> Result<Block> {
    let block_number = to_hex(block_number);
    let params = Params::Array(vec![Value::String(block_number), Value::Bool(true)]);
    let response = send("eth_getBlockByNumber", Some(params)).await?;
    let result: Block = serde_json::from_value(response)?;

    Ok(result)
}

/*
sample block from the chain
Object({
    "difficulty": String("0x0"),
    "extraData": String("0x"),
    "gasLimit": String("0x6691b7"),
    "gasUsed": String("0x0"),
    "hash": String("0x7a2b18ecb9565eaa511601130d8108886b5d9cb14c6f9662c1e661bbfc73523e"),
    "logsBloom": String("0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"),
    "miner": String("0x0000000000000000000000000000000000000000"),
    "mixHash": String("0x0000000000000000000000000000000000000000000000000000000000000000"),
    "nonce": String("0x0000000000000000"),
    "number": String("0x0"),
    "parentHash": String("0x0000000000000000000000000000000000000000000000000000000000000000"),
    "receiptsRoot": String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
    "sha3Uncles": String("0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"),
    "size": String("0x3e8"),
    "stateRoot": String("0xd5b2d8fdfe99430dcdaa397d252d0cae3a1457c414999fbba318ba90ec0ed56b"),
    "timestamp": String("0x60367687"),
    "totalDifficulty": String("0x0"),
    "transactions": Array([]),
    "transactionsRoot": String("0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"),
    "uncles": Array([])
})

*/

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_gets_a_block_number() {
        let response = get_block_number().await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn it_gets_the_zero_block() {
        let response = get_block(U64::from(0)).await;
        assert!(response.is_ok());
    }
}
