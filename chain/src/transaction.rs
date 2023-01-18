use crate::error::{ChainError, Result};

use blake2::{Blake2s256, Digest};
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use ethereum_types::{H256, U256, U64};
use proc_macros::NewType;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::convert::From;
use std::string::String;
use types::account::Account;
use types::block::BlockNumber;
use types::bytes::Bytes;
use types::transaction::{SimpleTransaction, SimpleTransactionReceipt};

#[derive(Serialize, Deserialize, Debug, NewType)]
pub(crate) struct Transaction(SimpleTransaction);

impl From<&Transaction> for SimpleTransactionReceipt {
    fn from(value: &Transaction) -> SimpleTransactionReceipt {
        SimpleTransactionReceipt {
            block_hash: value.hash,
            block_number: Some(BlockNumber(U64::zero())),
            contract_address: Some(value.to),
            transaction_hash: value.hash.unwrap(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct TransactionStorage {
    pub(crate) mempool: VecDeque<Transaction>,
    pub(crate) processed: DashMap<H256, Transaction>,
}

impl TransactionStorage {
    pub(crate) fn new() -> Self {
        Self {
            mempool: VecDeque::new(),
            processed: DashMap::new(),
        }
    }

    // add to the transaction mempool
    pub(crate) fn send_transaction(&mut self, transaction: Transaction) {
        self.mempool.push_back(transaction);
    }

    // get the transaction
    pub(crate) fn get_transaction(&self, hash: &H256) -> Result<Ref<H256, Transaction>> {
        let transaction = self
            .processed
            .get(hash)
            .ok_or_else(|| ChainError::TransactionNotFound(hash.to_string()))?;

        Ok(transaction)
    }

    // get the receipt of the transaction
    pub(crate) fn get_transaction_receipt(&self, hash: &H256) -> Result<SimpleTransactionReceipt> {
        let transaction = self.get_transaction(&hash)?;

        Ok(transaction.value().into())
    }
}

impl Transaction {
    pub(crate) fn new(
        from: Account,
        to: Account,
        value: U256,
        nonce: U256,
        data: Option<Bytes>,
    ) -> Self {
        let transaction = Self(SimpleTransaction {
            from,
            to,
            value,
            nonce,
            hash: None,
            data,
        });

        transaction.hash()
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

        Transaction::new(from, to, value, U256::zero(), None)
    }

    #[tokio::test]
    async fn creates_a_transaction() {
        let transaction = new_transaction();
        // println!("{:?}", transaction);
    }
}
