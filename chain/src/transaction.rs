use crate::error::{ChainError, Result};
use crate::helpers::serialize;

use blake2::{Blake2s256, Digest};
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
    pub(crate) processed: DashMap<H256, SimpleTransactionReceipt>,
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

    // get the receipt of the transaction
    pub(crate) fn get_transaction_receipt(&self, hash: &H256) -> Result<SimpleTransactionReceipt> {
        let transaction_receipt = self
            .processed
            .get(hash)
            .ok_or_else(|| ChainError::TransactionNotFound(hash.to_string()))
            .unwrap()
            .value()
            .clone();

        Ok(transaction_receipt)
    }
}

impl Transaction {
    pub(crate) fn new(
        from: Account,
        to: Account,
        value: U256,
        nonce: U256,
        data: Option<Bytes>,
    ) -> Result<Self> {
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

    pub(crate) fn serialize(&self) -> Result<Vec<u8>> {
        serialize(&self)
    }

    pub(crate) fn hash(mut self) -> Result<Self> {
        let hash = Blake2s256::digest(&self.serialize()?);
        self.hash = Some(H256::from(hash.as_ref()));

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn new_transaction() -> Transaction {
        let from = Account::random();
        let to = Account::random();
        let value = U256::from(1u64);

        Transaction::new(from, to, value, U256::zero(), None).unwrap()
    }

    #[tokio::test]
    async fn creates_a_transaction() {
        let transaction = new_transaction();
        // println!("{:?}", transaction);
    }
}
