//! # Web3
//!
//! This Web3 driver is a learning aid for understanding how real Web3 drivers
//! interact with Ethereum.

////////////////////////////////////////////////////////////////////////////////

mod chain;
mod error;
mod helpers;
mod method;
mod server;
mod state;

use error::Result;
use server::serve;
use state::State;

#[tokio::main]
async fn main() -> Result<()> {
    let state = State::new();
    let _server = serve("127.0.0.1:8545", state).await?;

    // create a future that never resolves
    futures::future::pending().await
}
