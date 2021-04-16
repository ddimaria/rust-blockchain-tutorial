use ethereum_types::{Address, Secret, U256};
use serde::{Deserialize, Serialize};

use crate::bytes::Bytes;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,
    pub gas: U256,
    pub gas_price: U256,
    pub data: Option<Bytes>,
}
