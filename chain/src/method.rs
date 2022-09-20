//! # Server
//!
//! Start the JsonRPC server and register methods

////////////////////////////////////////////////////////////////////////////////

use jsonrpsee::RpcModule;
use types::account::Account;

use crate::{error::Result, state::State};

pub(crate) fn eth_accounts(module: &mut RpcModule<State>) -> Result<()> {
    module.register_method("eth_accounts", |_, state| {
        let accounts = state.get_all_accounts();
        Ok(accounts)
    })?;

    Ok(())
}

pub(crate) fn eth_getBalance(module: &mut RpcModule<State>) -> Result<()> {
    module.register_method("eth_getBalance", move |params, state| {
        let key = params.one::<Account>()?;
        let account = state.get_account_balance(&key);
        Ok(account)
    })?;

    Ok(())
}

#[cfg(test)]
pub mod tests {
    use jsonrpsee::types::EmptyParams;
    use types::account::Account;

    use super::*;
    use crate::state::AccountData;

    #[tokio::test]
    async fn gets_all_accounts() {
        let state = State::new();
        let account_data = AccountData::new("123".into());
        let id = state.add_account(account_data);
        let mut module = RpcModule::new(state);
        eth_accounts(&mut module).unwrap();
        let response: Vec<Account> = module
            .call("eth_accounts", EmptyParams::new())
            .await
            .unwrap();

        assert_eq!(response, vec!(id));
    }

    #[tokio::test]
    async fn gets_an_account_balance() {
        let state = State::new();
        let account_data = AccountData::new("123".into());
        let tokens = account_data.tokens;
        let id = state.add_account(account_data);
        let mut module = RpcModule::new(state);
        eth_getBalance(&mut module).unwrap();
        let response: u64 = module.call("eth_getBalance", [id]).await.unwrap();

        assert_eq!(response, tokens);
    }
}
