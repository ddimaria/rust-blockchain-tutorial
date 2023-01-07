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
mod logger;
mod method;
mod server;
mod transaction;

use error::Result;
use helpers::tests::setup;
use server::serve;

#[tokio::main]
async fn main() -> Result<()> {
    // TODO(ddimaria): remove
    let (blockchain, _, _) = setup();
    let _server = serve("127.0.0.1:8545", blockchain).await?;

    // create a future that never resolves
    futures::future::pending().await
}
