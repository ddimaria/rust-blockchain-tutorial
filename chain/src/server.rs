//! # Server
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::{
    server::{ServerBuilder, ServerHandle},
    RpcModule,
};
use std::{env, net::SocketAddr, sync::Arc, time::Duration};
use tokio::{sync::Mutex, task, time};
use tracing_subscriber::{util::SubscriberInitExt, FmtSubscriber};

use crate::{
    blockchain::BlockChain,
    error::Result,
    logger::Logger,
    method::{
        eth_accounts, eth_block_number, eth_get_balance, eth_get_balance_by_block,
        eth_get_block_by_number, eth_get_code, eth_get_transaction_receipt,
        eth_send_raw_transaction, eth_send_transaction,
    },
};

pub(crate) type Context = Arc<Mutex<BlockChain>>;

// jsonrpsee requires static lifetimes for state
pub(crate) async fn serve(addr: &str, blockchain: Context) -> Result<ServerHandle> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    FmtSubscriber::builder().finish().try_init()?;

    let addrs = addr.parse::<SocketAddr>()?;
    let server = ServerBuilder::default()
        .set_logger(Logger)
        .build(addrs)
        .await?;
    let blockchain_for_transaction_processor = blockchain.clone();
    let mut module = RpcModule::new(blockchain);

    // register methods
    eth_accounts(&mut module)?;
    eth_block_number(&mut module)?;
    eth_get_block_by_number(&mut module)?;
    eth_get_balance(&mut module)?;
    eth_get_balance_by_block(&mut module)?;
    eth_send_transaction(&mut module)?;
    eth_send_raw_transaction(&mut module)?;
    eth_get_transaction_receipt(&mut module)?;
    eth_get_code(&mut module)?;

    let server_handle = server.start(module)?;

    tracing::info!("Starting server on {}", addrs);

    // process transactions in a separate thread
    let transaction_processor = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(1000));

        loop {
            interval.tick().await;

            blockchain_for_transaction_processor
                .lock()
                .await
                .process_transactions()
                .await;
        }
    });

    transaction_processor.await.unwrap();

    Ok(server_handle)
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::{core::client::ClientT, rpc_params};
    use types::account::Account;

    use crate::helpers::tests::{assert_vec_eq, client, server, setup};

    // #[tokio::test]
    // async fn creates_a_server() {
    //     let (blockchain, id_1, id_2) = setup().await;
    //     let server = server(Some(blockchain)).await;
    //     let response: Vec<Account> = client()
    //         .request("eth_accounts", rpc_params![])
    //         .await
    //         .unwrap();

    //     assert_vec_eq(response, vec![id_1, id_2]);
    // }
}
