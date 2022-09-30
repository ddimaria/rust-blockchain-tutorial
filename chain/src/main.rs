//! # Web3
//!
//! This Web3 driver is a learning aid for understanding how real Web3 drivers
//! interact with Ethereum.

////////////////////////////////////////////////////////////////////////////////

mod account;
mod block;
mod blockchain;
mod error;
mod helpers;
mod method;
mod server;
mod transaction;

use std::sync::{Arc, Mutex};

use blockchain::BlockChain;
use error::Result;
use server::serve;

#[tokio::main]
async fn main() -> Result<()> {
    let blockchain = BlockChain::new();
    let _server = serve("127.0.0.1:8545", blockchain).await?;

    // create a future that never resolves
    futures::future::pending().await
}
