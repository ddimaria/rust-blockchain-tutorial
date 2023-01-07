//! # Helpers
//!
//! General purpose utilties that don't have a home :(

////////////////////////////////////////////////////////////////////////////////

// #[cfg(test)]
pub mod tests {

    use std::sync::{Arc, Mutex};

    use ethereum_types::{H160, U256};
    use jsonrpsee::{
        http_client::{HttpClient, HttpClientBuilder},
        http_server::HttpServerHandle,
    };

    use crate::{
        account::AccountData, blockchain::BlockChain, server::serve, transaction::Transaction,
    };

    static ADDRESS: &'static str = "127.0.0.1:8545";

    pub(crate) async fn server(blockchain: Option<BlockChain>) -> HttpServerHandle {
        let blockchain = blockchain.unwrap_or_else(|| BlockChain::new());
        serve(ADDRESS, blockchain).await.unwrap()
    }

    pub(crate) fn client() -> HttpClient {
        let url = format!("http://{}", ADDRESS);
        HttpClientBuilder::default().build(url).unwrap()
    }

    pub(crate) fn setup() -> (BlockChain, H160, H160) {
        let mut blockchain = BlockChain::new();
        let account_data_1 = AccountData::new("123".into());
        let account_data_2 = AccountData::new("456".into());
        let id_1 = blockchain.accounts.add_account(account_data_1);
        let id_2 = blockchain.accounts.add_account(account_data_2);

        let value: ethereum_types::U256 = U256::from(1u64);
        let transaction = Transaction::new(id_1, id_2, value).hash();

        blockchain.new_block(vec![transaction.into()]);

        (blockchain, id_1, id_2)
    }

    pub(crate) fn assert_vec_eq<T: std::cmp::PartialEq>(vec_1: Vec<T>, vec_2: Vec<T>) {
        assert!(vec_1.iter().all(|item| vec_2.contains(item)));
    }
}
