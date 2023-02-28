use std::sync::Arc;

use eth_trie::{EthTrie, Trie};
use ethereum_types::{H256, U256};
use types::account::{Account, AccountData};
use types::block::BlockNumber;
use types::bytes::Bytes;
use utils::crypto::to_address;

use crate::helpers::{deserialize, serialize};
use crate::{
    error::{ChainError, Result},
    storage::Storage,
};

#[derive(Debug)]
pub(crate) struct AccountStorage {
    pub(crate) trie: EthTrie<Storage>,
}

impl AccountStorage {
    pub(crate) fn new(storage: Arc<Storage>) -> Self {
        Self {
            trie: EthTrie::new(Arc::clone(&storage)),
        }
    }

    pub(crate) fn upsert(&mut self, key: &Account, data: &AccountData) -> Result<()> {
        self.trie
            .insert(key.as_ref(), &serialize(&data)?)
            .map_err(|_| ChainError::StoragePutError(Storage::key_string(key)))
    }

    pub(crate) fn add_empty_account(&mut self, key: &Account) -> Result<()> {
        self.add_account(key, &AccountData::new(None))
    }

    pub fn add_contract_account(&mut self, key: &Account, data: Bytes) -> Result<Account> {
        let nonce = self.get_account(key)?.nonce;
        let serialized = bincode::serialize(&(key, nonce))?;
        let account = to_address(&serialized);
        let account_data = AccountData::new(Some(data));
        self.add_account(&account, &account_data)?;

        Ok(account)
    }

    pub(crate) fn add_account(&mut self, key: &Account, data: &AccountData) -> Result<()> {
        self.upsert(key, data)
    }

    pub(crate) fn get_account(&self, key: &Account) -> Result<AccountData> {
        let account = &self
            .trie
            .get(key.as_ref())
            .map_err(|_| ChainError::AccountNotFound(format!("Account {:?} not found", key)))?
            .ok_or_else(|| ChainError::StorageNotFound(Storage::key_string(key)))?;

        deserialize(account)
    }

    pub(crate) fn add_account_balance(&mut self, key: &Account, amount: U256) -> Result<()> {
        let mut account_data = self.get_account(key)?;
        account_data.balance += amount;
        self.upsert(key, &account_data)
    }

    pub(crate) fn subtract_account_balance(&mut self, key: &Account, amount: U256) -> Result<()> {
        let mut account_data = self.get_account(key)?;
        let balance = account_data.balance - amount;
        account_data.balance = std::cmp::max(U256::zero(), balance);
        self.upsert(key, &account_data)
    }

    pub(crate) fn transfer(&mut self, from: &Account, to: &Account, amount: U256) -> Result<()> {
        self.subtract_account_balance(from, amount)?;
        self.add_account_balance(to, amount)?;

        Ok(())
    }

    // TODO(ddimaria): remove
    pub(crate) fn increment_nonce(&mut self, key: &Account) -> Result<u64> {
        let mut account_data = self.get_account(key)?;
        account_data.nonce += 1;
        self.upsert(key, &account_data)?;

        Ok(account_data.nonce)
    }

    pub(crate) fn get_account_balance_by_block(
        &self,
        key: &Account,
        _block: &BlockNumber,
    ) -> Result<U256> {
        let balance = self.get_account(key)?.balance;
        Ok(balance)
    }

    pub(crate) fn get_nonce(&self, key: &Account) -> Result<u64> {
        let nonce = self.get_account(key)?.nonce;
        Ok(nonce)
    }

    pub(crate) fn hash_root(&mut self) -> Result<H256> {
        let root_hash = self
            .trie
            .root_hash()
            .map_err(|e| ChainError::CannotCreateRootHash(format!("account_trie: {}", e)))?;

        Ok(H256::from_slice(root_hash.as_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use ethereum_types::H160;

    use crate::helpers::tests::STORAGE;

    use super::*;

    fn new_account_storage() -> AccountStorage {
        AccountStorage::new((*STORAGE).clone())
    }

    fn add_account(account_storage: &mut AccountStorage) -> (AccountData, H160) {
        let account_data = AccountData::new(None);
        let key = Account::random();
        account_storage.add_account(&key, &account_data).unwrap();

        (account_data, key)
    }

    #[test]
    fn it_adds_and_gets_an_account() {
        let mut account_storage = new_account_storage();
        let (account_data, id) = add_account(&mut account_storage);
        let reteived_account_data = account_storage.get_account(&id).unwrap();
        assert_eq!(reteived_account_data, account_data);
    }

    #[test]
    fn it_increments_a_nonce() {
        let mut account_storage = new_account_storage();
        let (_, id) = add_account(&mut account_storage);
        let reteived_account_data = account_storage.get_account(&id).unwrap();
        assert_eq!(reteived_account_data.nonce, 0);

        account_storage.increment_nonce(&id).unwrap();
        let reteived_account_data = account_storage.get_account(&id).unwrap();
        dbg!(&reteived_account_data);
        assert_eq!(reteived_account_data.nonce, 1);
    }

    #[test]
    fn it_transfers() {
        let mut account_storage = new_account_storage();
        let (_, _) = add_account(&mut account_storage);
        let (_, _) = add_account(&mut account_storage);
    }

    #[test]
    fn hash_root_changes() {
        let mut account_storage = new_account_storage();
        let hash_root_1 = account_storage.hash_root().unwrap();
        let (_, _) = add_account(&mut account_storage);
        let hash_root_2 = account_storage.hash_root().unwrap();

        assert_ne!(hash_root_1, hash_root_2);
    }
}
