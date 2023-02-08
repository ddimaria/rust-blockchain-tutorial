use std::path::{Path, PathBuf};

use jsonrpsee::core::DeserializeOwned;
use rocksdb::DB;
use serde::Serialize;

use crate::error::{ChainError, Result};

const PATH: &str = ".tmp";
const DB: &str = "db";

pub(crate) fn db() -> DB {
    DB::open_default(path()).unwrap()
}

pub(crate) fn put<K: AsRef<[u8]>, V: Serialize>(db: &DB, key: K, value: &V) -> Result<()> {
    db.put(key, serialize(&value)?);
    Ok(())
}

pub(crate) fn get<K: AsRef<[u8]>, V: DeserializeOwned>(db: &DB, key: K) -> Result<V> {
    let value = db
        .get(&key)
        .map_err(|_| ChainError::StorageNotFound(key_string(&key)))?
        .ok_or_else(|| ChainError::StorageNotFound(key_string(&key)))?;
    deserialize(&value)
}

fn key_string<K: AsRef<[u8]>>(key: K) -> String {
    String::from_utf8(key.as_ref().to_vec()).unwrap_or("UNKNOWN".into())
}

fn serialize<V: Serialize>(value: &V) -> Result<Vec<u8>> {
    let serialized =
        bincode::serialize(value).map_err(|e| ChainError::StorageSerialize(e.to_string()))?;

    Ok(serialized)
}

fn deserialize<V: DeserializeOwned>(value: &Vec<u8>) -> Result<V> {
    let deserialized = bincode::deserialize::<V>(value)
        .map_err(|e| ChainError::StorageDeserialize(e.to_string()))?;

    Ok(deserialized)
}

fn path() -> PathBuf {
    Path::new(PATH).join(DB)
}

#[cfg(test)]
mod tests {
    use crate::account::AccountData;

    use super::*;

    #[test]
    fn it_creates_a_db() {
        let account_data = AccountData::new(None);
        let db = db();
        put(&db, b"k1", &account_data);
        println!("{:?}", get::<_, AccountData>(&db, b"k1"));
    }
}
