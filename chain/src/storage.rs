use std::path::{Path, PathBuf};

use jsonrpsee::core::DeserializeOwned;
use rocksdb::DB;
use serde::Serialize;

use crate::error::{ChainError, Result};

const PATH: &str = ".tmp";
const DATABASE_NAME: &str = "db";

#[derive(Debug)]
pub(crate) struct Storage {
    db: rocksdb::DB,
}

impl Storage {
    pub(crate) fn new() -> Result<Self> {
        let db = DB::open_default(Storage::path())
            .map_err(|e| ChainError::StorageCannotOpenDb(e.to_string()))?;

        Ok(Self { db })
    }

    pub(crate) fn insert<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: &V) -> Result<()> {
        self.db
            .put(&key, &Storage::serialize(&value)?)
            .map_err(|_| ChainError::StoragePutError(Storage::key_string(&key)))?;
        Ok(())
    }

    pub(crate) fn get<K: AsRef<[u8]>, V: DeserializeOwned>(&self, key: K) -> Result<V> {
        let value = self
            .db
            .get(&key)
            .map_err(|_| ChainError::StorageNotFound(Storage::key_string(&key)))?
            .ok_or_else(|| ChainError::StorageNotFound(Storage::key_string(&key)))?;

        Storage::deserialize(&value)
    }

    pub(crate) fn get_all_keys(&self) -> Result<Vec<Box<[u8]>>> {
        let value: Vec<Box<[u8]>> = self
            .db
            .iterator(rocksdb::IteratorMode::Start)
            .map(std::result::Result::unwrap)
            .map(|(key, _)| key)
            // .map(|(_, value)| Storage::deserialize(&*value))
            .collect();

        Ok(value)
    }

    pub(crate) fn contains_key<K: AsRef<[u8]>>(&self, key: K) -> bool {
        let pinned = self.db.get_pinned(&key);
        pinned.is_ok() && pinned.unwrap().is_some()
    }

    fn key_string<K: AsRef<[u8]>>(key: K) -> String {
        String::from_utf8(key.as_ref().to_vec()).unwrap_or("UNKNOWN".into())
    }

    fn serialize<V: Serialize>(value: &V) -> Result<Vec<u8>> {
        let serialized =
            bincode::serialize(value).map_err(|e| ChainError::StorageSerialize(e.to_string()))?;

        Ok(serialized)
    }

    fn deserialize<V: DeserializeOwned>(value: &[u8]) -> Result<V> {
        let deserialized = bincode::deserialize::<V>(value)
            .map_err(|e| ChainError::StorageDeserialize(e.to_string()))?;

        Ok(deserialized)
    }

    fn path() -> PathBuf {
        Path::new(PATH).join(DATABASE_NAME)
    }
}

#[cfg(test)]
mod tests {
    use types::account::Account;

    use crate::account::AccountData;
    use crate::helpers::tests::STORAGE;

    use super::*;

    #[test]
    fn it_creates_a_db() {
        let _ = STORAGE;
    }

    #[test]
    fn it_gets_and_insert_account_data_from_db() {
        let account = Account::random();
        let account_data = AccountData::new(None);
        STORAGE.insert(account, &account_data).unwrap();
        assert_eq!(
            account_data,
            STORAGE.get::<_, AccountData>(account).unwrap()
        );
    }
}
