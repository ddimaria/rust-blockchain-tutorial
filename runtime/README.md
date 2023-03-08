# WASM Runtime

The [runtime](runtime) crate is a wasmtime runtime for executing WASM contracts.
It leverages the [component model](https://github.com/WebAssembly/component-model) and [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) to simplify host and guest interactions.

## Development

Before running tests, you'll need to compile any contracts in the `/contracts` folder.

## Invoking a Contract Function

The `call_function!` macro invokes contract function calls with variable arguments:

```rust
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
```

For example, to invoke the `mint` function on the `erc20` contract:

```rust
let address = H160::from_str("0x5969c42d7f9ad971cb7fec4299e989cf308ca6f4")?;
call_function!(erc20, mint, address, 10);
```