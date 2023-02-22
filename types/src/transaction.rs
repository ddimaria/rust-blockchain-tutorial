//! # Transactions
//!
//! Accounts send transactions to the blockchain.
//! Within the blockchain, transactions are cryptographically signed.
//! Transactions live within blocks.
//!
//! see https://ethereum.org/en/developers/docs/transactions/

////////////////////////////////////////////////////////////////////////////////

use crypto::hash;
use ethereum_types::{Address, H160, H256, U256, U64};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::account::Account;
use crate::block::BlockNumber;
use crate::bytes::Bytes;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Transaction {
    pub data: Option<Bytes>,
    pub from: Address,
    pub gas: U256,
    pub gas_price: U256,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<H256>,
    pub nonce: U256,
    pub to: Address,
    pub value: U256,
}

impl Transaction {
    pub fn new(from: Account, to: Account, value: U256, nonce: U256, data: Option<Bytes>) -> Self {
        let mut transaction = Self {
            from,
            to,
            value,
            nonce,
            hash: None,
            data,
            gas: U256::from(10),
            gas_price: U256::from(10),
        };

        let serialized = bincode::serialize(&transaction).unwrap();
        let hashed: H256 = hash(&serialized).into();
        transaction.hash = Some(hashed);

        transaction
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SignedTransaction {
    pub v: u64,
    pub r: H256,
    pub s: H256,
    pub raw_transaction: Bytes,
    pub transaction_hash: H256,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct TransactionRequest {
    pub data: Option<Bytes>,
    pub gas: U256,
    pub gas_price: U256,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Option<U256>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r: Option<U256>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub s: Option<U256>,
}

impl From<Transaction> for TransactionRequest {
    fn from(value: Transaction) -> TransactionRequest {
        TransactionRequest {
            from: Some(value.from),
            to: Some(value.to),
            value: Some(value.value),
            data: value.data,
            gas: value.gas,
            gas_price: value.gas_price,
            r: None,
            s: None,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
// pub struct TransactionReceipt {
//     pub block_hash: Option<H256>,
//     pub block_number: Option<BlockNumber>,
//     pub contract_address: Option<H160>,
//     pub cumulative_gas_used: U256,
//     pub gas_used: Option<U256>,
//     pub logs: Vec<Log>,
//     pub logs_bloom: Bloom,
//     pub root: Option<H256>,
//     pub status: Option<U64>,
//     pub transaction_hash: H256,
//     pub transaction_index: String,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct TransactionReceipt {
    pub block_hash: Option<H256>,
    pub block_number: Option<BlockNumber>,
    pub contract_address: Option<H160>,
    pub transaction_hash: H256,
}

impl From<&Transaction> for TransactionReceipt {
    fn from(value: &Transaction) -> TransactionReceipt {
        TransactionReceipt {
            block_hash: value.hash,
            block_number: Some(BlockNumber(U64::zero())),
            contract_address: Some(value.to),
            transaction_hash: value.hash.unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
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
