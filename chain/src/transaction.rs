use blake2::{Blake2s256, Digest};
use ethereum_types::{H256, U256};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::ops::{Deref, DerefMut};
use std::string::String;
use types::account::Account;
use types::bytes::Bytes;
use types::transaction::{SimpleTransaction, TransactionRequest};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Transaction(SimpleTransaction);

// Deref and DerefMut make working with newtypes easy
impl Deref for Transaction {
    type Target = SimpleTransaction;

    fn deref(&self) -> &SimpleTransaction {
        &self.0
    }
}

impl DerefMut for Transaction {
    fn deref_mut(&mut self) -> &mut SimpleTransaction {
        &mut self.0
    }
}

impl From<&TransactionRequest> for Transaction {
    fn from(value: &TransactionRequest) -> Transaction {
        Transaction::new(value.from.unwrap(), value.to.unwrap(), value.value.unwrap())
    }
}

impl Transaction {
    pub(crate) fn new(from: Account, to: Account, value: U256) -> Self {
        Self(SimpleTransaction {
            from,
            to,
            value,
            nonce: U256::zero(), // only one transaction per block for now
            hash: None,
        })
    }

    pub(crate) fn serialize(&self) -> String {
        format!("{:?}", (&self.nonce, &self.from, &self.to, &self.value))
    }

    pub(crate) fn hash(mut self) -> Self {
        let hash = Blake2s256::digest(&self.serialize());
        self.hash = Some(H256::from(hash.as_ref()));
        self
    }

    pub(crate) fn is_signed(&self) -> bool {
        self.hash.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn new_transaction() -> Transaction {
        let from = Account::random();
        let to = Account::random();
        let value = U256::from(1u64);
        Transaction::new(from, to, value).hash()
    }

    #[tokio::test]
    async fn creates_a_transaction() {
        let transaction = new_transaction();
        println!("{:?}", transaction);
    }
}
