# Rust Blockchain Rutorial

_WORK IN PROGRESS_

This repo is designed to train entry-level Rust developers on intermediate and advanced Rust development in the context of Ethereum blockchain development.
While learning Rust, the developer will also explore Ethereum concepts and implement a naive Web3 driver, a rpc client, and even build a simple blockchain.

## Organization

### Web3

The [web3](web3) crate is a naive implementation of a Web3 interface.
It has been minimized to focus on learning the concepts of the blockchain.
The goal will be to build out some of the most used endpoints.

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
