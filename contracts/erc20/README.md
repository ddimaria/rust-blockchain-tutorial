# ERC20 WASM Contract

The [contracts](contracts) directory holds the WASM source code.
Using [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen), we can greatly simplify dealing with complex types.
The [WIT format](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) specifies a language for generating WASM code. 

## Contracts

### WIT

```wit
default world contract {
  export erc20: interface {
    construct: func(name: string, symbol: string)
    mint: func(account: string, amount: u64)
    transfer: func(to: string, amount: u64)
  }
}
```

### Erc20

Using the magical `generate!` macro, we remove boilerplate glue code, so all you see is the Rust contract. 

```rust
use wit_bindgen_guest_rust::*;

wit_bindgen_guest_rust::generate!({path: "../erc20/erc20.wit", world: "erc20"});

struct Erc20 {}

export_contract!(Erc20);

impl erc20::Erc20 for Erc20 {
    fn construct(name: String, symbol: String) {
        println!("name {}, symbol", symbol);
    }

    fn mint(account: String, amount: u64) {
        println!("account {}, amount", amount);
    }

    fn transfer(to: String, amount: u64) {
        println!("to {}, amount", amount);
    }
}
```

## Build
```shell
cargo build --target wasm32-unknown-unknown --release
```