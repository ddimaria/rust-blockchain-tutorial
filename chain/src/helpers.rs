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

    use std::{str::FromStr, sync::Arc};

    use ethereum_types::{H160, H256, U256};
    use jsonrpsee::{
        http_client::{HttpClient, HttpClientBuilder},
        server::ServerHandle,
    };
    use lazy_static::lazy_static;
    use rocksdb::{DBCommon, SingleThreaded};
    use tokio::sync::Mutex;
    use types::account::{Account, AccountData};
    use types::transaction::Transaction;

    use crate::{blockchain::BlockChain, server::serve, storage::Storage};

    static ADDRESS: &str = "127.0.0.1:8545";
    static DATABASE_NAME: Option<&str> = Some("test");

    lazy_static! {
        pub(crate) static ref STORAGE: Arc<Storage> =
            Arc::new(Storage::new(DATABASE_NAME).unwrap());
        pub(crate) static ref ACCOUNT_1: Account =
            H160::from_str("0x4a0d457e884ebd9b9773d172ed687417caac4f14").unwrap();
        pub(crate) static ref ACCOUNT_2: Account = Account::random();
        pub(crate) static ref ACCOUNT_3: Account = Account::random();
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
        // Storage::destroy(DATABASE_NAME).unwrap();
        let mut blockchain = BlockChain::new((*STORAGE).clone()).unwrap();
        let mut account_data_1 = AccountData::new(None);

        account_data_1.balance = U256::from(100_000);

        blockchain
            .accounts
            .add_account(&ACCOUNT_1, &account_data_1)
            .unwrap();

        let value: ethereum_types::U256 = U256::from(1u64);

        // let transaction = Transaction::new(
        //     *ACCOUNT_1,
        //     Some(*ACCOUNT_2),
        //     value,
        //     Some(*ACCOUNT_1_NONCE.lock().await),
        //     None,
        // )
        // .unwrap();

        // blockchain.new_block(vec![transaction], H256::zero());

        (Arc::new(Mutex::new(blockchain)), *ACCOUNT_1, *ACCOUNT_2)
    }

    pub(crate) fn assert_vec_contains<T: std::cmp::PartialEq>(vec_1: Vec<T>, vec_2: Vec<T>) {
        assert!(vec_2.iter().all(|item| vec_1.contains(item)));
    }
}
