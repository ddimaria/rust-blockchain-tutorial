use dashmap::DashMap;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use types::account::Account;

use crate::{block::Block, blockchain::BlockChain};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub(crate) struct AccountData {
    pub(crate) password: String,
    pub(crate) tokens: u64,
}

impl AccountData {
    pub(crate) fn new(password: String) -> Self {
        Self {
            password,
            tokens: 0,
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

    pub(crate) fn add_account(&self, data: AccountData) -> Account {
        let key = Account::random();

        if !self.accounts.contains_key(&key) {
            self.accounts.insert(key, data);
        }

        key
    }

    pub(crate) fn get_all_accounts(&self) -> Vec<Account> {
        self.accounts
            .par_iter_mut()
            .map(|ref_mut_multi| ref_mut_multi.key().to_owned())
            .collect()
    }

    pub(crate) fn get_account(&self, key: &Account) -> Option<AccountData> {
        let account_data = self.accounts.get(key)?.value().to_owned();
        Some(account_data)
    }

    pub(crate) fn get_account_balance(&self, key: &Account) -> Option<u64> {
        let tokens = self.get_account(key)?.tokens;
        Some(tokens)
    }

    pub(crate) fn get_account_balance_by_block(
        &self,
        key: &Account,
        _block: &Block,
    ) -> Option<u64> {
        let tokens = self.get_account(key)?.tokens;
        Some(tokens)
    }
}
