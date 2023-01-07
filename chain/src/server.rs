//! # Server
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::{
    http_server::{HttpServerBuilder, HttpServerHandle},
    RpcModule,
};
use std::{env, net::SocketAddr};
use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

use crate::{
    blockchain::BlockChain,
    error::Result,
    logger::Logger,
    method::{eth_accounts, eth_get_balance, eth_get_balance_by_block, eth_send_transaction},
};

pub(crate) type Context = BlockChain;

// jsonrpsee requires static lifetimes for state
pub(crate) async fn serve(addr: &str, blockchain: Context) -> Result<HttpServerHandle> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    FmtSubscriber::builder().finish().try_init()?;

    let addrs = addr.parse::<SocketAddr>()?;
    let server = HttpServerBuilder::default()
        .set_middleware(Logger)
        .build(addrs)
        .await?;
    let mut module = RpcModule::new(blockchain);

    // register methods
    eth_accounts(&mut module)?;
    eth_get_balance(&mut module)?;
    eth_get_balance_by_block(&mut module)?;
    eth_send_transaction(&mut module)?;

    let server_handle = server.start(module)?;

    tracing::info!("Starting server on {}", addrs);

    Ok(server_handle)
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::core::client::ClientT;
    use types::account::Account;

    use crate::helpers::tests::{assert_vec_eq, client, server, setup};

    #[tokio::test]
    async fn creates_a_server() {
        let (blockchain, id_1, id_2) = setup();
        let _server = server(Some(blockchain)).await;
        let response: Vec<Account> = client().request("eth_accounts", None).await.unwrap();

        assert_vec_eq(response, vec![id_1, id_2]);
    }
}
