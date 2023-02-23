//! # Helpers
//!
//! General purpose utilties that don't have a home :(

////////////////////////////////////////////////////////////////////////////////

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{
    ChainError::{DeserializeError, SerializeError},
    Result,
};

pub(crate) fn serialize<V: Serialize>(value: &V) -> Result<Vec<u8>> {
    let serialized = bincode::serialize(value).map_err(|e| SerializeError(e.to_string()))?;
    Ok(serialized)
}

pub(crate) fn deserialize<V: DeserializeOwned>(value: &[u8]) -> Result<V> {
    let deserialized =
        bincode::deserialize::<V>(value).map_err(|e| DeserializeError(e.to_string()))?;

    Ok(deserialized)
}

// #[cfg(test)]
#[allow(unused)]
pub mod tests {

    use std::sync::Arc;

    use ethereum_types::{H160, U256};
    use jsonrpsee::{
        http_client::{HttpClient, HttpClientBuilder},
        server::ServerHandle,
    };
    use lazy_static::lazy_static;
    use rocksdb::{DBCommon, SingleThreaded};
    use tokio::sync::Mutex;
    use types::transaction::Transaction;

    use crate::{account::AccountData, blockchain::BlockChain, server::serve, storage::Storage};

    static ADDRESS: &'static str = "127.0.0.1:8545";

    lazy_static! {
        pub(crate) static ref STORAGE: Arc<Storage> = Arc::new(Storage::new(Some("test")).unwrap());
    }

    pub(crate) async fn server(blockchain: Option<Arc<Mutex<BlockChain>>>) -> ServerHandle {
        let blockchain = blockchain
            .unwrap_or_else(|| Arc::new(Mutex::new(BlockChain::new((*STORAGE).clone()).unwrap())));
        serve(ADDRESS, blockchain).await.unwrap()
    }

    pub(crate) fn client() -> HttpClient {
        let url = format!("http://{}", ADDRESS);
        HttpClientBuilder::default().build(url).unwrap()
    }

    pub(crate) async fn setup() -> (Arc<Mutex<BlockChain>>, H160, H160) {
        let mut blockchain = BlockChain::new((*STORAGE).clone()).unwrap();
        let account_data_1 = AccountData::new(None);
        let account_data_2 = AccountData::new(None);
        let id_1 = blockchain
            .accounts
            .add_account(None, &account_data_1)
            .unwrap();
        let id_2 = blockchain
            .accounts
            .add_account(None, &account_data_2)
            .unwrap();
        blockchain.accounts.add_account_balance(&id_1, 100).unwrap();

        let value: ethereum_types::U256 = U256::from(1u64);
        let transaction = Transaction::new(id_1, id_2, value, U256::zero(), None).unwrap();

        blockchain.new_block(vec![transaction]);

        (Arc::new(Mutex::new(blockchain)), id_1, id_2)
    }

    pub(crate) fn assert_vec_contains<T: std::cmp::PartialEq>(vec_1: Vec<T>, vec_2: Vec<T>) {
        assert!(vec_2.iter().all(|item| vec_1.contains(item)));
    }
}
