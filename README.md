# Rust Blockchain Tutorial

_WORK IN PROGRESS_

This repo is designed to train entry-level Rust developers on intermediate and advanced Rust development in the context of Ethereum blockchain development.
While learning Rust, the developer will also explore Ethereum concepts and implement a naive Web3 driver, a rpc client, and even build a simple blockchain.

## Roadmap

- [x] Ethereum Types
- [x] Basic Chain Node
- [x] Basic Web3 Client
- [x] WASM/WASI VM for Contract Execution (wasmtime)
- [ ] P2P Networking between Nodes (libp2p)
- [ ] PoS Consensus
- [x] Persistent Disk Chain State (RocksDB)
- [ ] Intermediate Chain Node
- [ ] Intermediate Web3 Client
- [ ] Full Tutorial
- [ ] CI

## Organization

### Chain

The [chain](chain) crate is a simplistic ethereum blockchain node.
It currently holds state in memory (TBD on disk storage).
The external json-rpc API mirrors that of Ethereum.
It contains a WASM runtime for executing contracts.

#### Sample API: eth_blockNumber

##### Request
```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_accounts","params":[]}' \ 
     http://127.0.0.1:8545
```

##### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":[
        "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
        "0x70997970c51812dc3a010c7d01b50e0d17dc79c8",
        "0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc"
    ]
}
```

The full API can be found in the chain [README](chain).

### Runtime

The [runtime](runtime) crate is a wasmtime runtime for executing WASM contracts.
It leverages the [component model](https://github.com/WebAssembly/component-model) and [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) to simplify host and guest interactions.

### Contracts

The [contracts-wasm](contracts-wasm) directory holds the WASM source code.
Using [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen), we can greatly simplify dealing with complex types.

#### WIT

The [WIT format](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) specifies a language for generating WASM code. 

```wit
default world contract {
  export erc20: interface {
    mint: func(address: string, amount: u64)
  }
}
```

#### Sample Contract - Erc20

Using the magical `generate!` macro, we remove boilerplate glue code, so all you see is the Rust contract. 

```rust
use wit_bindgen_guest_rust::*;

wit_bindgen_guest_rust::generate!({path: "../erc20/erc20.wit", world: "erc20"});

struct Erc20 {}

export_contract!(Erc20);

impl erc20::Erc20 for Erc20 {
    fn mint(address: String, amount: u64) {
        // mint some coin
    }
}
```

### Web3

The [web3](web3) crate is a naive implementation of a Web3 interface.
It has been minimized to focus on learning the concepts of the blockchain.
The goal will be to build out some of the most used endpoints.

#### Sample Usage

```rust
use web3::Web3;

let web3 = Web3::new("http://127.0.0.1:8545")?;
let all_accounts = web3.get_all_accounts().await;
let balance = web3.get_balance(all_accounts[0]).await;

let block_number = web3.get_block_number().await?;
let block = web3.get_block(*block_number).await?;

let contract =
    include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let tx_hash = web3.deploy(all_accounts[0], &contract).await?;
let receipt = web3.transaction_receipt(tx_hash).await?;
let code = web3.code(receipt.contract_address.unwrap(), None).await?;
```

More information can be found in the web3 [README](web3).

### Types

The [types](types) crate is holds shared types to be used by the other crates.

## Getting Started

First, start the chain:

```shell
cd chain
RUST_LOG=info cargo run
```

You should see:

```console
2023-01-25T00:58:58.382776Z  INFO chain::server: Starting server on 127.0.0.1:8545
```

You can now send [json-rpc calls](web3) to the API.

## Compiling

```rust
cargo build
```

## Running Tests

```rust
cargo test
```
