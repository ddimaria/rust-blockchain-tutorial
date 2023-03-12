use crate::error::{Result, RuntimeError};
use wasmtime::{
    self,
    component::{Component, Instance, Linker, Val},
    Config, Engine, Store,
};
use wit_component::ComponentEncoder;

pub fn call_function(bytes: &[u8], function: &str, params: &[&str]) -> Result<()> {
    let (mut store, instance) = load_contract(bytes)?;
    let parsed: Result<Vec<Val>> = params.chunks_exact(2).map(parse_params).collect();

    tracing::info!("{} params {:?}", function, parsed);

    let function = instance
        .get_func(&mut store, function)
        .ok_or_else(|| RuntimeError::ExportFunctionError(function.into()))?;

    function
        .call(&mut store, &parsed?, &mut [])
        .map_err(|e| RuntimeError::CallFunctionError(e.to_string()))
}

fn load_contract(bytes: &[u8]) -> Result<(Store<i32>, Instance)> {
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
    let instance = linker.instantiate(&mut store, &component)?;

    Ok((store, instance))
}

// TODO(ddimaria): remove unwrap
fn parse_params(chunk: &[&str]) -> Result<Val> {
    match chunk[0] {
        "String" => Ok(Val::String(chunk[1].into())),
        "U64" => Ok(Val::U64(chunk[1].parse::<u64>().unwrap())),
        _ => Err(RuntimeError::InvalidParamType(chunk[0].into())),
    }
}

// for debugging exportable functions
fn contract_functions(bytes: &[u8]) -> Vec<String> {
    let mut config = Config::new();
    let mut exports = vec![];

    Config::wasm_component_model(&mut config, true);

    if let Ok(engine) = Engine::new(&config) {
        exports = wasmtime::Module::from_binary(&engine, bytes)
            .unwrap()
            .exports()
            .map(|export| export.name().to_string())
            .collect();
    }

    exports
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
    use types::account::Account;

    const PARAMS_1: &[&str] = &["String", "Rust Coin", "String", "RustCoin"];

    fn params_2<'a>(address: &'a String) -> [&'a str; 4] {
        ["String", &address, "U64", "10"]
    }

    #[test]
    fn it_loads_a_contract() {
        let bytes = include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20_wit.wasm");
        let _loaded = load_contract(bytes).unwrap();
    }

    #[test]
    fn it_calls_contract_functions() {
        let bytes = include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20_wit.wasm");
        let address = Account::random().to_string();

        call_function(bytes, "construct", PARAMS_1).unwrap();
        call_function(bytes, "mint", &params_2(&address)).unwrap();
    }

    #[test]
    fn it_parses_string_params() {
        let parsed = parse_params(&[PARAMS_1[0], PARAMS_1[1]]).unwrap();
        assert_eq!(parsed, Val::String("Rust Coin".into()));
    }

    #[test]
    fn it_parses_u64_params() {
        let address = Account::random().to_string();
        let params = params_2(&address);
        let parsed = parse_params(&[params[2], params[3]]).unwrap();
        assert_eq!(parsed, Val::U64(10));
    }

    #[test_log::test]
    fn it_retrieves_contract_function_names() {
        let bytes = include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20_wit.wasm");
        let functions = contract_functions(bytes);
        let expected = [
            "memory",
            "construct",
            "mint",
            "transfer",
            "cabi_realloc",
            "__data_end",
            "__heap_base",
        ];

        assert_eq!(functions, expected);
    }
}
