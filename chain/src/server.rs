//! # Server
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::{
    http_server::{HttpServerBuilder, HttpServerHandle},
    RpcModule,
};
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

use crate::{
    blockchain::BlockChain,
    error::Result,
    method::{eth_accounts, eth_get_balance, eth_send_transaction},
};

pub(crate) type Context = BlockChain;

// jsonrpsee requires static lifetimes for state
pub(crate) async fn serve(addr: &str, blockchain: Context) -> Result<HttpServerHandle> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    FmtSubscriber::builder().finish().try_init()?;

    let addrs = addr.parse::<SocketAddr>()?;
    let server = HttpServerBuilder::default().build(addrs).await?;
    let mut module = RpcModule::new(blockchain);

    eth_accounts(&mut module)?;
    eth_get_balance(&mut module)?;
    eth_send_transaction(&mut module)?;

    let server_handle = server.start(module)?;

    tracing::info!("Starting server on {}", addrs);

    Ok(server_handle)
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::core::client::ClientT;
    use types::account::Account;

    use super::*;
    use crate::account::AccountData;
    use crate::helpers::tests::{client, server};

    #[tokio::test]
    async fn creates_a_server() {
        let blockchain = BlockChain::new();
        let account_data = AccountData::new("123".into());
        let id = blockchain.accounts.add_account(account_data);
        let _server = server(Some(blockchain)).await;
        let response: Vec<Account> = client().request("eth_accounts", None).await.unwrap();

        assert_eq!(response, vec!(id));
    }
}
