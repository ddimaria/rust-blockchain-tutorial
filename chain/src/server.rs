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
    error::Result,
    method::{eth_accounts, eth_getBalance},
    state::State,
};

// jsonrpsee requires static lifetimes for state
pub(crate) async fn serve(addr: &str, state: State) -> Result<HttpServerHandle> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }

    FmtSubscriber::builder().finish().try_init()?;

    let addrs = addr.parse::<SocketAddr>()?;
    let server = HttpServerBuilder::default().build(addrs).await?;
    let mut module = RpcModule::new(state);

    eth_accounts(&mut module)?;
    eth_getBalance(&mut module)?;

    let server_handle = server.start(module)?;

    tracing::info!("Starting server on {}", addrs);

    Ok(server_handle)
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::core::client::ClientT;
    use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
    use types::account::Account;

    use crate::state::AccountData;

    use super::*;

    static ADDRESS: &'static str = "127.0.0.1:8545";

    pub(crate) async fn server(state: Option<State>) -> HttpServerHandle {
        let state = state.unwrap_or_else(|| State::new());
        serve(ADDRESS, state).await.unwrap()
    }

    pub(crate) fn client() -> HttpClient {
        let url = format!("http://{}", ADDRESS);
        HttpClientBuilder::default().build(url).unwrap()
    }

    #[tokio::test]
    async fn creates_a_server() {
        let state = State::new();
        let account_data = AccountData::new("123".into());
        let id = state.add_account(account_data);
        let _server = server(Some(state)).await;
        let response: Vec<Account> = client().request("eth_accounts", None).await.unwrap();

        assert_eq!(response, vec!(id));
    }
}
