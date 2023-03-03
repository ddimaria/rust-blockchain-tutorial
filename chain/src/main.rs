//! # Web3
//!
//! This Web3 driver is a learning aid for understanding how real Web3 drivers
//! interact with Ethereum.

////////////////////////////////////////////////////////////////////////////////

mod account;
mod blockchain;
mod error;
mod helpers;
mod keys;
mod logger;
mod method;
mod server;
mod storage;
mod transaction;
mod world_state;

// use std::sync::Arc;

// use blockchain::BlockChain;
use error::Result;
use server::serve;
// use storage::Storage;
// use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO(ddimaria): remove hard-coded values and implement a genesis file (or via command line when starting the chain)
    // let storage = Arc::new(Storage::new(None)?);
    // let blockchain = BlockChain::new(storage)?;
    // let _server = serve("127.0.0.1:8545", Arc::new(Mutex::new(blockchain))).await?;

    let (blockchain, _, _) = crate::helpers::tests::setup().await;
    let _server = serve("127.0.0.1:8545", blockchain).await?;

    // create a future that never resolves
    futures::future::pending().await
}
