//! # Helpers
//!
//! General purpose utilties that don't have a home :(

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
pub mod tests {

    use std::sync::{Arc, Mutex};

    use jsonrpsee::{
        http_client::{HttpClient, HttpClientBuilder},
        http_server::HttpServerHandle,
    };

    use crate::{blockchain::BlockChain, server::serve};

    static ADDRESS: &'static str = "127.0.0.1:8545";

    pub(crate) async fn server(blockchain: Option<BlockChain>) -> HttpServerHandle {
        let blockchain = blockchain.unwrap_or_else(|| BlockChain::new());
        serve(ADDRESS, blockchain).await.unwrap()
    }

    pub(crate) fn client() -> HttpClient {
        let url = format!("http://{}", ADDRESS);
        HttpClientBuilder::default().build(url).unwrap()
    }
}
