# Rust Blockchain Tutorial

_WORK IN PROGRESS_

This repo is designed to train entry-level Rust developers on intermediate and advanced Rust development in the context of Ethereum blockchain development.
While learning Rust, the developer will also explore Ethereum concepts and implement a naive Web3 driver, a rpc client, and even build a simple blockchain.

## Roadmap

- [x] Ethereum Types
- [x] Basic Chain Node
- [x] Basic Web3 Client
- [ ] WASM VM for Contract Execution (wasmtime)
- [ ] P2P Networking between Nodes (libp2p)
- [ ] PoS Consensus
- [ ] Persistent Disk Chain State (RocksDB)
- [ ] Intermediate Chain Node
- [ ] Intermediate Web3 Client
- [ ] Full Tutorial
- [ ] CI

## Organization

### Chain

The [chain](chain) crate is a simplistic ethereum blockchain. The goal is to implement major features to fully integrate with the [web3](web3) crate.

This chain currently has in-memory state

#### API

##### eth_blockNumber

###### Request
```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_accounts","params":[]}' \ 
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":[
        0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266,
        0x70997970c51812dc3a010c7d01b50e0d17dc79c8,
        0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc
    ]
}
```

More information can be found in the web3 [README](chain).

### Web3

The [web3](web3) crate is a naive implementation of a Web3 interface.
It has been minimized to focus on learning the concepts of the blockchain.
The goal will be to build out some of the most used endpoints.

Sample usage:

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

### Client

The [client](client) crate is a working ethereum rpc client. This may or may not access a custom blockchain, time permitting.

### Contracts

The [contracts](contracts) directory holds our source and compiled contracts. For now, there is just an ERC20 contract in there, but hope to add ERC721 and ERC1155 later.

## Getting Started

TBD

See individual crates for instructions.

## Compiling

```rust
cargo build
```

## Running Tests

```rust
cargo test
```
