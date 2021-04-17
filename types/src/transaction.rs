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
    pub data: Option<Bytes>,
    pub from: Address,
    pub gas: U256,
    pub gas_price: U256,
    pub hash: Secret,
    pub nonce: U256,
    pub to: Address,
    pub value: U256,
}

#[skip_serializing_none]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    pub data: Option<Bytes>,
    pub gas: U256,
    pub gas_price: U256,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Option<U256>,
}

#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionReceipt {
    pub block_hash: Option<H256>,
    pub block_number: Option<BlockNumber>,
    pub contract_address: Option<H160>,
    pub cumulative_gas_used: U256,
    pub gas_used: Option<U256>,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
    pub root: Option<H256>,
    pub status: Option<U64>,
    pub transaction_hash: H256,
    pub transaction_index: String,
}

#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Log {
    pub address: H160,
    pub block_hash: Option<H256>,
    pub block_number: Option<U64>,
    pub data: Bytes,
    pub log_index: Option<U256>,
    pub log_type: Option<String>,
    pub removed: Option<bool>,
    pub topics: Vec<H256>,
    pub transaction_hash: Option<H256>,
    pub transaction_index: Option<String>,
    pub transaction_log_index: Option<U256>,
}
