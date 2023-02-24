<!-- omit in toc -->
# Rust Blockchain Tutorial

_WORK IN PROGRESS_

This repo is designed to train Rust developers on intermediate and advanced Rust development and help understand the primary concepts in Ethereum blockchain development.

<!-- omit in toc -->
## Roadmap

- [x] Ethereum Types
- [x] Cryptography Primitives
- [x] Chain Node
- [x] Web3 Client
- [x] WASM/WASI VM for Contract Execution (wasmtime)
- [ ] Rust Smart Contracts
  - [x] Base Implementation
  - [ ] Fungible
  - [ ] Non Fungible
  - [ ] Multi Asset
- [ ] P2P Networking between Nodes (libp2p)
- [ ] PoS Consensus
- [x] Persistent Disk Chain State (RocksDB)
- [ ] Full Tutorial
- [ ] CI

<!-- omit in toc -->
## Table of Contents

- [Introduction](#introduction)
- [Ethereum Primitives](#ethereum-primitives)
  - [Accounts](#accounts)
  - [Transactions](#transactions)
- [Organization](#organization)
  - [Chain](#chain)
    - [Sample API: eth\_blockNumber](#sample-api-eth_blocknumber)
      - [Request](#request)
      - [Response](#response)
  - [Runtime](#runtime)
  - [Contracts](#contracts)
    - [WIT](#wit)
    - [Sample Contract - Erc20](#sample-contract---erc20)
  - [Web3](#web3)
    - [Sample Usage](#sample-usage)
  - [Types](#types)
  - [Crypto](#crypto)
- [Getting Started](#getting-started)
- [Compiling](#compiling)
- [Running Tests](#running-tests)

## Introduction

When I first entered the crypto space, I had never used Rust and was unfamiliar with blockchain technology, let alone any knowledge of Ethereum concepts.  This is the tutorial I wish I had back then, and will hopefully guide Rust developers along their blockchain journey.

While the concepts explored here are based on Ethereum, there are many instances where they diverge from it.  For example, structs are simplified to just show general concepts.  Different hashing and consensus algorithims are implemented.  The most divergent area are smart contracts.  We'll explore Rust-based smart contracts that run on a WASM virtual machine.  I went in this direction to keep the language choice homogenious.  The overall approach is similar to Solidity, though the implementation is very different.

## Ethereum Primitives

### Accounts

In Ethereum, `Accounts` are either `Externally Owned Accounts` or `Contract Accounts`.  Addresses are hex encoded: `0x71562b71999873DB5b286dF957af199Ec94617F7`.

```rust
type Account = ethereum_types::Address;
```

For a given address, data associated with the account is stored on chain:

```rust
struct AccountData {
    nonce: u64,
    balance: u64,
    code_hash: Option<Bytes>,
}

impl AccountData {
    fn is_contract(&self) -> bool {
        self.code_hash.is_some()
    }
}
```

Externally Owned Accounts are simply a public address.  This address is a 20 byte hash (`H160`), and is created by applying a hash function on the public key, taking the last 20 bytes.  This is how we create an Ethereum Account in Rust:

```rust
use crypto::{keypair, public_key_address};

let (private_key, public_key) = keypair();
let address = public_key_address(&public_key);

fn public_key_address(key: &PublicKey) -> H160 {
    let public_key = key.serialize_uncompressed();
    let hash = hash(&public_key[1..]);

    Address::from_slice(&hash[12..])
}
```

Public keys are not stored on the chain.  Since we can't derive the public key from the hash, the public key is not known until a signed transaction is validated.  We'll dig a bit more into this in the Transaction section.

Contract Accounts are also just an address, but have a code hash associated with them.  A contract's address is created by encoding the sender's address and their current nonce.  This encoding is then hashed using a hash function, taking the last 20 bytes.  This process is similiar to the Externally Owned Account creation, but the input is an encoded tuple.

```rust
use crypto::{to_address};
use web3::web3;

let account = MY_ACCOUNT_ADDRESS;
let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let nonce = web3().get_transaction_count(account).await.unwrap();
let serialized: Vec<u8> = bincode::serialize(&(account, nonce)).unwrap();
let contract_address = to_address(&serialized).unwrap();
```

It's important to note that addresses (accounts) are iniatiated outside of a blockchain.  They can be generated in many ways, though the most common is to use a wallet.  In our examples, we'll sign them offline using the provided tools in the `crypto` crate.  Accounts are stored on the chain when they are used for the first time.

Accounts are also deterministic.  That is, given the same inputs, the same address is always generated.

### Transactions

Transactions are the heart of a blockchain.  Without them, the chain's state would remain unchanged.  Transactions drive state changes.

```rust
pub struct Transaction {
    pub data: Option<Bytes>,
    pub from: Address,
    pub gas: U256,
    pub gas_price: U256,
    pub hash: Option<H256>,
    pub nonce: U256,
    pub to: Address,
    pub value: U256,
}
```

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

The [contracts](contracts) directory holds the WASM source code.
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

The [types](types) crate holds shared types to be used by the other crates.

### Crypto

The [crypto](crypto) crate provides functions for generating keys, hashing data, signing and verifying.

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
