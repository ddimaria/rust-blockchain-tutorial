//! # Helpers
//!
//! General purpose utilties that don't have a home :(

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub(crate) mod tests {
    use std::str::FromStr;

    use crate::Web3;
    use ethereum_types::{H160, U256};
    use lazy_static::lazy_static;
    use tokio::sync::Mutex;
    use types::account::Account;

    lazy_static! {
        pub(crate) static ref ACCOUNT_1: Account =
            H160::from_str("0x4a0d457e884ebd9b9773d172ed687417caac4f14").unwrap();
        pub(crate) static ref ACCOUNT_2: Account = Account::random();
        pub(crate) static ref ACCOUNT_1_NONCE: Mutex<U256> = Mutex::new(U256::zero());
    }

    pub fn web3() -> Web3 {
        Web3::new("http://127.0.0.1:8545").unwrap()
    }

    pub fn get_contract() -> Vec<u8> {
        include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20_wit.wasm").to_vec()
    }
}
