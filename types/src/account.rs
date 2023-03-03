//! # Accounts
//!
//! In Ethereum, Accounts are just addresses.
//! Accounts can have ETH and tokens, and can send transactions to the chain.
//! A deployed contract is also an account.
//! Accounts can also interact with deployed contracts.
//!
//! see https://ethereum.org/en/developers/docs/accounts/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::{Address, U256};
use serde::{Deserialize, Serialize};

use crate::bytes::Bytes;

pub type Account = Address;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AccountData {
    pub nonce: U256,
    pub balance: U256,
    pub code_hash: Option<Bytes>,
}

impl AccountData {
    pub fn new(code_hash: Option<Bytes>) -> Self {
        Self {
            nonce: U256::zero(),
            balance: U256::zero(),
            code_hash,
        }
    }

    pub fn _is_contract(&self) -> bool {
        self.code_hash.is_some()
    }
}
