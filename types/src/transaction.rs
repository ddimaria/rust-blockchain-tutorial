use ethereum_types::{Address, Secret, U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub nonce: U256,
    pub hash: Secret,
    pub gas: U256,
    pub gas_price: U256,
    pub data: Option<Vec<u8>>,
}
