#![allow(unused)]
use crate::error::Result;
use paste::paste;
use std::collections::HashMap;
use types::account::Account;
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};
use wit_component::ComponentEncoder;

wasmtime::component::bindgen!({ path: "../contracts/erc20/erc20.wit", world: "erc20" });

macro_rules! call_function {
    ($contract: ident, $function: ident, $($args:tt),*) => {{
        let bytes = include_bytes!(concat!(
            "./../../target/wasm32-unknown-unknown/release/",
            stringify!($contract),
            "_wit.wasm"
        ));
        paste!{
            if let Ok((mut store, contract)) = load_contract(bytes) {
                [contract. $contract . $function(&mut store, $($args),*)];
            };
        }
    }};
}

fn load_contract(bytes: &[u8]) -> Result<(Store<i32>, Contract)> {
    let mut config = Config::new();

    Config::wasm_component_model(&mut config, true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, 0);
    let linker = Linker::new(&engine);

    let component_bytes = ComponentEncoder::default()
        .module(bytes)?
        .validate(true)
        .encode()?;
    let component = Component::from_binary(&engine, &component_bytes)?;
    let (contract, _) = Contract::instantiate(&mut store, &component, &linker)?;

    Ok((store, contract))
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test_log::test;
    use types::account::Account;

    #[test_log::test]
    fn it_calls_a_contract_function() {
        let address = &Account::random().to_string();
        call_function!(erc20, construct, "Rust Coin", "RustCoin");
        call_function!(erc20, mint, address, 10);
    }
}
