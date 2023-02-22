use crate::error::{ChainError, Result};

use dashmap::DashMap;
use ethereum_types::H256;
use std::collections::VecDeque;
use types::transaction::{Transaction, TransactionReceipt};

#[derive(Debug)]
pub(crate) struct TransactionStorage {
    pub(crate) mempool: VecDeque<Transaction>,
    pub(crate) processed: DashMap<H256, TransactionReceipt>,
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
    pub(crate) fn get_transaction_receipt(&self, hash: &H256) -> Result<TransactionReceipt> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ethereum_types::U256;
    use std::convert::From;
    use types::account::Account;

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
