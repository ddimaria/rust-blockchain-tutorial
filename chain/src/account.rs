use std::sync::Arc;

use serde::{Deserialize, Serialize};
use types::account::Account;
use types::block::BlockNumber;
use types::bytes::Bytes;

use crate::{
    error::{ChainError, Result},
    storage::Storage,
};

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

    pub(crate) fn is_contract(&self) -> bool {
        self.code_hash.is_some()
    }
}

#[derive(Debug)]
pub(crate) struct AccountStorage {
    pub(crate) accounts: Arc<Storage>,
}

impl AccountStorage {
    pub(crate) fn new(storage: Arc<Storage>) -> Self {
        Self { accounts: storage }
    }

    pub(crate) fn add_account(&self, key: Option<Account>, data: &AccountData) -> Result<Account> {
        let key = key.unwrap_or_else(|| Account::random());

        if !self.accounts.contains_key(&key) {
            self.accounts.insert(key, &data)?;
        } else {
            tracing::info!("Did not create account {} as it already exists", &key);
        }

        Ok(key)
    }

    pub(crate) fn add_account_balance(&self, key: &Account, amount: u64) -> Result<()> {
        self.get_account(&key)?.balance += amount;
        Ok(())
    }

    pub(crate) fn get_all_accounts(&self) -> Vec<Account> {
        self.accounts
            .get_all_keys()
            .unwrap_or_else(|_| vec![])
            .iter()
            .map(|value| Account::from_slice(value.as_ref()))
            .collect()
    }

    pub(crate) fn get_account(&self, key: &Account) -> Result<AccountData> {
        self.accounts
            .get(key)
            .map_err(|_| ChainError::AccountNotFound(format!("Account {} not found", key)))
    }

    pub(crate) fn increment_nonce(&mut self, key: &Account) -> Result<u64> {
        let mut account_data = self.get_account(&key)?;
        account_data.nonce += 1;
        self.accounts.update(key, &account_data)?;

        Ok(account_data.nonce)
    }

    pub(crate) fn get_account_balance_by_block(
        &self,
        key: &Account,
        _block: &BlockNumber,
    ) -> Result<u64> {
        let balance = self.get_account(key)?.balance;
        Ok(balance)
    }

    pub(crate) fn get_nonce(&self, key: &Account) -> Result<u64> {
        let nonce = self.get_account(key)?.nonce;
        Ok(nonce)
    }
}

#[cfg(test)]
mod tests {
    use ethereum_types::H160;

    use crate::helpers::tests::{assert_vec_contains, STORAGE};

    use super::*;

    fn new_account_storage() -> AccountStorage {
        AccountStorage::new((*STORAGE).clone())
    }

    fn add_account(account_storage: &AccountStorage) -> (AccountData, H160) {
        let account_data = AccountData::new(None);
        let id = account_storage.add_account(None, &account_data).unwrap();

        (account_data, id)
    }

    #[test]
    fn it_adds_and_gets_an_account() {
        let account_storage = new_account_storage();
        let (account_data, id) = add_account(&account_storage);
        let reteived_account_data = account_storage.get_account(&id).unwrap();
        assert_eq!(reteived_account_data, account_data);
    }

    #[test]
    fn it_increments_a_nonce() {
        let mut account_storage = new_account_storage();
        let (_, id) = add_account(&account_storage);
        let reteived_account_data = account_storage.get_account(&id).unwrap();
        assert_eq!(reteived_account_data.nonce, 0);

        account_storage.increment_nonce(&id).unwrap();
        let reteived_account_data = account_storage.get_account(&id).unwrap();
        assert_eq!(reteived_account_data.nonce, 1);
    }

    #[test]
    fn it_gets_all_accounts() {
        let account_storage = new_account_storage();
        let (_, id_1) = add_account(&account_storage);
        let (_, id_2) = add_account(&account_storage);
        let accounts = account_storage.get_all_accounts();
        assert_vec_contains(accounts, vec![id_1, id_2]);
    }
}
