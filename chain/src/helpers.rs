//! # Helpers
//!
//! General purpose utilties that don't have a home :(

////////////////////////////////////////////////////////////////////////////////

// #[cfg(test)]
#[allow(unused)]
pub mod tests {

    use std::sync::Arc;

    use ethereum_types::{H160, U256};
    use jsonrpsee::{
        http_client::{HttpClient, HttpClientBuilder},
        server::ServerHandle,
    };
    use tokio::sync::Mutex;

    use crate::{
        account::AccountData, blockchain::BlockChain, server::serve, storage::db,
        transaction::Transaction,
    };

    static ADDRESS: &'static str = "127.0.0.1:8545";

    pub(crate) async fn server(blockchain: Option<Arc<Mutex<BlockChain>>>) -> ServerHandle {
        let blockchain = blockchain.unwrap_or_else(|| Arc::new(Mutex::new(BlockChain::new(db()))));
        serve(ADDRESS, blockchain).await.unwrap()
    }

    pub(crate) fn client() -> HttpClient {
        let url = format!("http://{}", ADDRESS);
        HttpClientBuilder::default().build(url).unwrap()
    }

    pub(crate) async fn setup() -> (Arc<Mutex<BlockChain>>, H160, H160) {
        let mut blockchain = BlockChain::new(db());
        let account_data_1 = AccountData::new(None);
        let account_data_2 = AccountData::new(None);
        let id_1 = blockchain.accounts.add_account(None, account_data_1);
        let id_2 = blockchain.accounts.add_account(None, account_data_2);
        blockchain.accounts.add_account_balance(&id_1, 100).unwrap();

        let value: ethereum_types::U256 = U256::from(1u64);
        let transaction = Transaction::new(id_1, id_2, value, U256::zero(), None).hash();

        blockchain.new_block(vec![transaction.into()]);

        (Arc::new(Mutex::new(blockchain)), id_1, id_2)
    }

    pub(crate) fn assert_vec_eq<T: std::cmp::PartialEq>(vec_1: Vec<T>, vec_2: Vec<T>) {
        assert!(vec_1.iter().all(|item| vec_2.contains(item)));
    }
}
