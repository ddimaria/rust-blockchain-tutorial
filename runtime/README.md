# WASM Runtime

The [runtime](runtime) crate is a wasmtime runtime for executing WASM contracts.
It leverages the [component model](https://github.com/WebAssembly/component-model) and [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) to simplify host and guest interactions.

## Development

Before running tests, you'll need to compile any contracts in the `/contracts` folder.

## Invoking a Contract Function

This code can convert the textual representation of a contract function call to a function call within the wasmtime runtime.
Parameters are listed in pairs of parameter type and paramater value.

```rust
let bytes = include_bytes!("./../../target/wasm32-unknown-unknown/release/erc20_wit.wasm");
let function_name = "construct";
let params = &["String", "Rust Coin", "String", "RustCoin"];

call_function(bytes, function_name, params)?;
```

## Types

To conform with the WASM Component Model, the following types are supported:

* Bool(bool)
* S8(i8)
* U8(u8)
* S16(i16)
* U16(u16)
* S32(i32)
* U32(u32)
* S64(i64)
* U64(u64)
* Float32(f32)
* Float64(f64)
* Char(char)
* String(Box<str>)
* List(List)
* Record(Record)
* Tuple(Tuple)
* Variant(Variant)
* Enum(Enum)
* Union(Union)
* Option(OptionVal)
* Result(ResultVal)
* Flags(Flags)