//! # Transactions
//!
//! Accounts send transactions to the blockchain.
//! Within the blockchain, transactions are cryptographically signed.
//! Transactions live within blocks.
//!
//! see https://ethereum.org/en/developers/docs/transactions/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::{Address, Bloom, Secret, H160, H256, U256, U64};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::block::BlockNumber;
use crate::bytes::Bytes;

#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub nonce: U256,
    pub hash: Secret,
    pub gas: U256,
    pub gas_price: U256,
    pub data: Option<Bytes>,
}

#[skip_serializing_none]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Option<U256>,
    pub gas: U256,
    pub gas_price: U256,
    pub data: Option<Bytes>,
}

#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionReceipt {
    pub transaction_hash: H256,
    pub transaction_index: String,
    pub block_hash: Option<H256>,
    pub block_number: Option<BlockNumber>,
    pub cumulative_gas_used: U256,
    pub gas_used: Option<U256>,
    pub contract_address: Option<H160>,
    pub logs: Vec<Log>,
    pub status: Option<U64>,
    pub root: Option<H256>,
    pub logs_bloom: Bloom,
}

#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub address: H160,
    pub topics: Vec<H256>,
    pub data: Bytes,
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub transaction_hash: Option<H256>,
    pub transaction_index: Option<String>,
    pub log_index: Option<U256>,
    pub transaction_log_index: Option<U256>,
    pub log_type: Option<String>,
    pub removed: Option<bool>,
}
