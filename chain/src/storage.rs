use std::path::{Path, PathBuf};

use eth_trie::DB as EthDB;
use rocksdb::{Options, DB};

use crate::error::{ChainError, Result};

const PATH: &str = ".tmp";
const DATABASE_NAME: &str = "db";

#[derive(Debug)]
pub(crate) struct Storage {
    db: rocksdb::DB,
}

/// Implement a patricia merkle trie interface to work directly with RocksDB
impl EthDB for Storage {
    type Error = ChainError;

    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        let value = self
            .db
            .get(key)
            .map_err(|_| ChainError::StorageNotFound(Storage::key_string(key)))?;

        Ok(value)
    }

    fn insert(&self, key: &[u8], value: Vec<u8>) -> Result<()> {
        self.db
            .put(key, value)
            .map_err(|_| ChainError::StoragePutError(Storage::key_string(key)))?;

        Ok(())
    }

    // noop
    fn remove(&self, _key: &[u8]) -> Result<()> {
        Ok(())
    }

    // noop
    fn flush(&self) -> Result<()> {
        Ok(())
    }
}

impl Storage {
    pub(crate) fn new(database_name: Option<&str>) -> Result<Self> {
        let database_name = database_name.unwrap_or(DATABASE_NAME);
        let db = DB::open_default(Storage::path(database_name))
            .map_err(|e| ChainError::StorageCannotOpenDb(e.to_string()))?;

        Ok(Self { db })
    }

    pub(crate) fn _get_all_keys<K: AsRef<[u8]>>(&self) -> Result<Vec<Box<[u8]>>> {
        let value: Vec<Box<[u8]>> = self
            .db
            .iterator(rocksdb::IteratorMode::Start)
            .map(std::result::Result::unwrap)
            .map(|(key, _)| key)
            .collect();

        Ok(value)
    }

    pub(crate) fn _destroy(database_name: Option<&str>) -> Result<()> {
        let database_name = database_name.unwrap_or(DATABASE_NAME);
        DB::destroy(&Options::default(), Storage::path(database_name))
            .map_err(|e| ChainError::StorageDestroyError(e.into()))?;

        Ok(())
    }

    pub(crate) fn key_string<K: AsRef<[u8]>>(key: K) -> String {
        String::from_utf8(key.as_ref().to_vec()).unwrap_or_else(|_| "UNKNOWN".into())
    }

    fn path(database_name: &str) -> PathBuf {
        Path::new(PATH).join(database_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::{deserialize, serialize, tests::STORAGE};
    use eth_trie::DB;
    use types::account::{Account, AccountData};

    #[test]
    fn it_creates_a_db() {
        let _ = STORAGE;
    }

    #[test]
    fn it_gets_and_insert_account_data_from_db() {
        let account = Account::random();
        let account_data = AccountData::new(None);
        STORAGE
            .insert(account.as_ref(), serialize(&account_data).unwrap())
            .unwrap();

        let retrieved = STORAGE.get(account.as_ref()).unwrap().unwrap();

        assert_eq!(account_data, deserialize(&retrieved).unwrap());
    }
}
