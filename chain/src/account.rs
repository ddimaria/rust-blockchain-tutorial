use dashmap::mapref::one::RefMut;
use dashmap::DashMap;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use types::account::Account;
use types::block::BlockNumber;
use types::bytes::Bytes;

use crate::error::{ChainError, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct AccountData {
    pub(crate) nonce: u64,
    pub(crate) balance: u64,
    pub(crate) code_hash: Option<Bytes>,
}

impl AccountData {
    pub(crate) fn new(code_hash: Option<Bytes>) -> Self {
        Self {
            nonce: 0,
            balance: 0,
            code_hash,
        }
    }
}

#[derive(Debug)]
pub(crate) struct AccountStorage {
    pub(crate) accounts: DashMap<Account, AccountData>,
}

impl AccountStorage {
    pub(crate) fn new() -> Self {
        Self {
            accounts: DashMap::new(),
        }
    }

    pub(crate) fn add_account(&self, key: Option<Account>, data: AccountData) -> Account {
        let key = key.unwrap_or_else(|| Account::random());

        if !self.accounts.contains_key(&key) {
            self.accounts.insert(key, data);
        }

        key
    }

    pub(crate) fn add_account_balance(&self, key: &Account, amount: u64) -> Result<()> {
        self.get_mut_account(&key)?.balance += amount;

        Ok(())
    }

    pub(crate) fn get_all_accounts(&self) -> Vec<Account> {
        self.accounts
            .par_iter_mut()
            .map(|ref_mut_multi| ref_mut_multi.key().to_owned())
            .collect()
    }

    pub(crate) fn get_account(&self, key: &Account) -> Result<AccountData> {
        let account_data = self.get_mut_account(&key)?.value().to_owned();
        Ok(account_data)
    }

    pub(crate) fn get_mut_account(&self, key: &Account) -> Result<RefMut<Account, AccountData>> {
        self.accounts
            .get_mut(key)
            .ok_or_else(|| ChainError::AccountNotFound(format!("Account {} not found", key)))
    }

    pub(crate) fn increment_nonce(&mut self, key: &Account) -> Result<u64> {
        let mut account = self.get_mut_account(&key)?;
        account.nonce += 1;

        Ok(account.nonce)
    }

    pub(crate) fn get_account_balance(&self, key: &Account) -> Result<u64> {
        let balance = self.get_account(key)?.balance;
        Ok(balance)
    }

    pub(crate) fn get_account_balance_by_block(
        &self,
        key: &Account,
        _block: &BlockNumber,
    ) -> Result<u64> {
        let balance = self.get_account(key)?.balance;
        Ok(balance)
    }
}
