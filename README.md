# Rust Blockchain Rutorial

This repo is designed to train entry-level Rust developers on intermediate and advanced Rust development in the context of Ethereum blockchain development.
While learning Rust, the developer will also explore Ethereum concepts and implement a naive Web3 driver, a rpc client, and even build a simple blockchain.

## Organization

### Client

The [client](client) crate is a working ethereum rpc client.

### Contracts

The [contracts](contracts) directory holds our source and compiled contracts.

### Types

The [types](types) crate is holds shared types to be use by the binaries and libraries.

### Web3

The [web3](web3) crate is a naive implementation of a Web3 interface. It has been minimized to focus on learning the concepts of the blockchain.

## Getting Started

TBD

## Compiling

```rust
cargo build
```

## Running Tests

```rust
cargo test
```
