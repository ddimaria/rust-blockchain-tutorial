use crate::error::Result;
use wasmtime::{
    self,
    component::{Component, Linker},
    Config, Engine, Store,
};
use wit_component::ComponentEncoder;

wasmtime::component::bindgen!({ path: "../contracts/erc20/erc20.wit", world: "erc20" });

fn deploy<'a>(bytes: &[u8], name: &str, symbol: &str) -> Result<()> {
    // let (mut store, contract) = load_contract(bytes)?;
    // let res = contract.erc20.mint(&mut store, name, symbol)?;

    // assert_eq!(res, "RustCoin");
    // println!("Out: {}", res);
    Ok(())
}

fn mint<'a>(bytes: &[u8], address: &str, amount: u64) -> Result<()> {
    let (mut store, contract) = load_contract(bytes)?;
    let res = contract.erc20.mint(&mut store, address, amount)?;

    // assert_eq!(res, "RustCoin");
    // println!("Out: {}", res);
    Ok(res)
}

fn load_contract<'a>(bytes: &[u8]) -> Result<(Store<i32>, Contract)> {
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
    fn it_mints() {
        mint(
            include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20_wit.wasm"),
            &Account::random().to_string(),
            100 as u64,
        )
        .unwrap();
    }
}
