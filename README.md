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
    - [Nonce](#nonce)
  - [Transactions](#transactions)
    - [Kinds of Transactions](#kinds-of-transactions)
    - [Transaction Hashes](#transaction-hashes)
    - [Transaction Signing](#transaction-signing)
  - [Blocks](#blocks)
    - [Genesis Block](#genesis-block)
- [Basic Cryptography](#basic-cryptography)
  - [Public Key Cryptography](#public-key-cryptography)
  - [Hashing](#hashing)
  - [Address](#address)
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

#### Nonce

Accounts have an associated `nonce`.  A nonce is an acronym for "number only used once" and is a counter of the number of processed transactions for a given account.  It is to be incremented every time a new transaction is submitted.  User's must keep track of their own nonces, though wallet providers can do this as well.  Anyone can query the blockchain for current nonce of an account (EOA and contract), which can be helpful for determining the next nonce to use.

The main purpose of a nonce is to make a data structure unique, so that each data structure is explicit regarding otherwise identical data being effectively different.  We'll discuss nonces more when breaking down transactions and how blockchain nodes use them to preserve the order of processing submitted transactions.

### Transactions

Transactions are the heart of a blockchain.  Without them, the chain's state would remain unchanged.  Transactions drive state changes.  They are submitted externally owned accounts only (i.e. not a contract account).

```rust
pub struct Transaction {
    pub from: Address,
    pub to: Option<Address>,
    pub hash: Option<H256>,
    pub nonce: U256,
    pub value: U256,
    pub data: Option<Bytes>,
    pub gas: U256,
    pub gas_price: U256,
}
```

While the `Transaction` data structure has much more fields in Ethereum than shown above, the data subset we're using is the minimum needed to understand transactions.

* The `from` portion of a transaction identifies the transaction sender.  This account must already exist on the blockchain.
* The `to` attribute represents the receiver of the value transferred in a transaction.  It can also be a contract address, where code is executed.  It is optional because it is left empty (or zero or null) to signify a transaction that deploys a contract.
* A `hash` attribute contains the hash of the transaction.  It's optional so that it can be calculated after the other values in the transaction are populated.
* The `nonce` is the sender's next account nonce.  It is the existing account nonce incremented by one.  Multiple transactions submitted consecutively must incremented manually as the current account nonce won't change on chain until the transactions are processed.
* `value` indicates the amount of `coin` to transfer from the sender to the recipient.  This number can be zero for non-value-transferring transactions.
* The `data` attribute can hold various pieces of data.  When deploying a contract, it holds bytes of the assembled contract code.  When executing a function on a contract, it holds the function name and parameters.  It can also be any piece of data that the sender wants to include in the transaction.
* `gas` is the total number of units that the sender is offering to pay for the transaction.  We'll discuss this in more detail later.
* The `gas_price` is the amount of `coin` (eth in Ethereum) to be paid for each unit of `gas`.

#### Kinds of Transactions

There are 3 ways that transaction can be used:

```rust

pub enum TransactionKind {
    Regular(Address, Address),
    ContractDeployment(Address, Bytes),
    ContractExecution(Address, Address, Bytes),
}
```

* `Regular` transactions are ones where value is transferred from one account to another.
* `Contract deployment` transactions are used to deploy contract code to the blockchain and are without a 'to' address, where the data field is used for the contract code.
* `Contract execution` transactions interact with a deployed smart contract. In this case, 'to' address is the smart contract address.

The type of transaction is derived from the values in the transaction:

```rust
fn kind(self) -> Result<TransactionKind> {
    match (self.from, self.to, self.data) {
        (from, Some(to), None) => Ok(TransactionKind::Regular(from, to)),
        (from, None, Some(data)) => Ok(TransactionKind::ContractDeployment(from, data)),
        (from, Some(to), Some(data)) => Ok(TransactionKind::ContractExecution(from, to, data)),
        _ => Err(TypeError::InvalidTransaction("kind".into())),
    }
}
```

#### Transaction Hashes

Once a transaction data structure is filled in, the hash can be calculated:

```rust
let serialized = bincode::serialize(&transaction)?;
let hash: H256 = hash(&serialized).into();
```

We first encode/serialize the transaction and then apply a hashing function.  To keep things simple, we're using [Bincode](https://github.com/bincode-org/bincode) to serialize and compress the data to a binary format throughout this blockchain.  Ethereum uses [RLP Encoding](https://ethereum.org/en/developers/docs/data-structures-and-encoding/rlp/) for most of it's encoding/serialization.

#### Transaction Signing

Transactions must be signed before they are submitted to the blockchain.

```rust
fn sign(&self, key: SecretKey) -> Result<SignedTransaction> {
    let encoded = bincode::serialize(&self)?;
    let recoverable_signature = sign_recovery(&encoded, &key)?;
    let (_, signature_bytes) = recoverable_signature.serialize_compact();
    let Signature { v, r, s } = recoverable_signature.into();
    let transaction_hash = hash(&signature_bytes).into();

    let signed_transaction = SignedTransaction {
        v,
        r,
        s,
        raw_transaction: encoded.into(),
        transaction_hash,
    };

    Ok(signed_transaction)
}
```

Signing is done with the account holder's private key in order to generate a recoverable signature of the transaction.  We'll discuss basic cryptography in more detail later, but a recoverable signature is one where a public key can be derived from the signature and message.  This is important as it's how the blockchain validates the transaction.  Once the public key is recovered, it is hashed and must match the `from` address of the transaction.

The resulting `SignedTransaction` data structure is represented as:

```rust
struct SignedTransaction {
    v: u64,
    r: H256,
    s: H256,
    raw_transaction: Bytes,
    transaction_hash: H256,
}
```

The `v`, `r`, and `s` values represent the digital signature.  The `v` attribute is the recovery id that is used to derive the account holder's public key.  `r` and `s` hold values related to the signature (`r` is the value and `s` is the proof).

The transaction encoded and compressed and is stored as bytes in the `raw_transaction` attribute.  This minimizes the footprint of the packet.

The `transaction_hash` will be the transaction id in the blockchain.  It serves many purposes, and can be used to validate that the reconstructed transaction wasn't tampered with.

### Blocks

Blocks essentially containers for validated transactions.  Once a set of transactions are validated, they are ready to be added to a block.

```rust
struct Block {
    number: U64,
    hash: Option<H256>,
    parent_hash: H256,
    transactions: Vec<Transaction>,
    transactions_root: H256,
    state_root: H256,
}
```

A Block `number` is an incremented unsigned integer.  All blocks are in order, and there are no missing blocks.  Blocks start at zero (called the `Genesis Block`).  Similar to transactions, blocks have a `hash` of their values:

```rust
let serialized = bincode::serialize(&block)?;
let hash: H256 = hash(&serialized).into();
block.hash = Some(hash);
```

The `parent_hash` is the hash of the previous block.  This links blocks together and are critical in blockchain validation.  All transactions are within the block and are needed for the block verification process.

The `transaction_root` is the Merkle Root of all of the transactions in the block:

```rust
fn to_trie(transactions: &[Transaction]) -> Result<EthTrie<MemoryDB>> {
    let memdb = Arc::new(MemoryDB::new(true));
    let mut trie = EthTrie::new(memdb);

    transactions.iter().try_for_each(|transaction| {
        trie.insert(
            transaction.transaction_hash()?.as_bytes(),
            bincode::serialize(&transaction)?,
        )?
    })?;

    Ok(trie)
}

fn root_hash(transactions: &[Transaction]) -> Result<H256> {
    let mut trie = Transaction::to_trie(transactions)?;
    let root_hash = trie.root_hash()?;

    Ok(H256::from_slice(root_hash.as_bytes()))
}
```

The `state_root` is the Merkle Root of all state within the blockchain.  We'll detail this more when discussing Global State.

#### Genesis Block

The first block in a blockchain is called the `genesis block`.  We're using a naive implementation to create an empty block with no state:

```rust
fn genesis() -> Result<Self> {
    Block::new(U64::zero(), H256::zero(), vec![], H256::zero())
}
```

In Ethereum, the genesis block is created using a `genisis file`.  This file contains configuration information and details the accounts to create and the amount of Eth they each get.  This is where the initial Eth comes from, though additional Eth is created for miners.  The movement to Proof of Stake in Ethereum V2 transforms miners to validators, and changes the reward mechanism.  We'll talk more about this in the consensus section.

## Basic Cryptography

We've talked about public and private keys, addresses, and hashes, but dive a little deeper into what they are.

### Public Key Cryptography

Public Key Cryptography is asymetric encryption, where there are public and private keys (key pair).  This is different from symmetric encryption that uses the same key to encrypt and decrypt.

There are differnt flavors of public key cryptography.  In this project we're using [Secp256k1](https://en.bitcoin.it/wiki/Secp256k1), which works with [ECDSA](https://en.bitcoin.it/wiki/Elliptic_Curve_Digital_Signature_Algorithm) signatures.  Secp256k1 is an elliptic curve.  We're not going to get into anything that complicated here, so just picture a line that's shaped like a door knob.  The public and private keys are just points on the curve.

The public key is derived from the private key.  A key is just a number (unsigned 256-bit in our case).  While you can derive a public key from a private key, you cannot create a private key from a public one.

Here's how the encryption works.  Bob wants to send Alice a secret message.  He first asks Alice for her public key, which he uses to encrypt the message.  He then sends the message to Alice and she uses her private key to decrypt the message.  If anyone intercepts the message, they can't read it unless they have Alice's private key.  Alice knows that the message was not altered and she can prove that Bob sent her a message.

```rust
use utils::crypto::keypair;

let (private_key, public_key) = keypair();
println!("{:?}", keypair());

fn keypair() -> (SecretKey, PublicKey) {
    generate_keypair(&mut rand::thread_rng())
}
```

Outputs:

```txt
SecretKey(#2fd3a5f2b1f52597)
PublicKey(6dfc30040b48fefe3e3cebe0ca2d5033189a93b50aeaa1876d21ea15fc756b6e3e8e48997fb1441464e254877fe898f6566488d2b0d578f2fbd31b48772be593)
```

### Hashing

The concept of a hash is fairly straightforward.  A hash function accepts any size data and outputs a number of fixed length.  Anytime the input is the same, the output (e.g. the hash) will always be the same.  Another key component is that you cannot derive the original data from the hash.

Hashes are used everywhere in blockchains.  They allow the formal verification that the source data was used to create it.  They are convenient ways of storing a proof of the input, without the burden of storing the data.

```rust
let public_key = 6dfc30040b48fefe3e3cebe0ca2d5033189a93b50aeaa1876d21ea15fc756b6e3e8e48997fb1441464e254877fe898f6566488d2b0d578f2fbd31b48772be593;
let hash = hash(&public_key[1..]);
println!("{:?}", hash);
```

Outputs:

```txt
[101, 35, 44, 44, 55, 13, 158, 100, 34, 209, 215, 186, 89, 105, 196, 45, 127, 154, 217, 113, 203, 127, 236, 66, 153, 233, 137, 207, 48, 140, 166, 244]
```

### Address

We already know that an account on Ethereum is just an address, but what is an address?  It's simply the trailing 20 bytes from a hash.

```rust
let public_key = 6dfc30040b48fefe3e3cebe0ca2d5033189a93b50aeaa1876d21ea15fc756b6e3e8e48997fb1441464e254877fe898f6566488d2b0d578f2fbd31b48772be593;
let hash = hash(&public_key[1..]);
let address = Address::from_slice(&hash[12..]);
println!("{:?}", address);
```

Outputs:

```txt
0x5969c42d7f9ad971cb7fec4299e989cf308ca6f4
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
    construct: func(name: string, symbol: string)
    mint: func(account: string, amount: u64)
    transfer: func(to: string, amount: u64)
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
