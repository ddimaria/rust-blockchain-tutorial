# Web3

_NOTE: not for use in production_

This Web3 driver is a learning aid for understanding how real Web3 drivers interact with Ethereum.

This crate interacts with the Hardhat chain in the [contracts](../contracts) directory.

## Accounts

### Get All Accounts

```rust
use web3::account::get_all_accounts;

let all_accounts = get_all_accounts().await;
```

### Get Account Balance

```rust
use web3::account::{get_all_accounts, get_balance};

let account = get_all_accounts().await.unwrap()[0].clone();
let balance = get_balance(account).await;
```

### Get Account Balance from a Block

```rust
use types::block::BlockNumber;
use web3::account::{get_all_accounts, get_balance_by_block};

let block = BlockNumber(0.into());
let account = get_all_accounts().await.unwrap()[0];
let balance = get_balance_by_block(account, Some(block)).await;
```

## Blocks

### Get Current Block Number

```rust
use web3::block::get_block_number;

let block_number = get_block_number()).await;
```

### Retrieve a Block

```rust
use web3::block::get_block;

let block_number = U64::from(0);
let block = get_block(block_number)).await;
```

## Contracts

### Deploy a Contract

```rust
use web3::account::get_all_accounts;
use web3::contract::deploy;

let account = get_all_accounts().await.unwrap()[0];
let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let tx_hash = deploy(account, &contract).await;
```

## Transactions

### Send a Transaction

```rust
use web3::account::get_all_accounts;
use web3::transaction::send;

let from = get_all_accounts().await.unwrap()[0];
let to = get_all_accounts().await.unwrap()[0];
let gas = U256::from(1_000_000);
let gas_price = U256::from(1);
let data = get_contract();
let transaction_request = TransactionRequest {
    from: None,
    to: Some(to),
    value: None,
    gas,
    gas_price,
    data: Some(data.into()),
};
let tx_hash = send(transaction_request).await;
```

## Other Work

For a full-blown crate that you can use in production, check out the official [Web3](https://github.com/tomusdrw/rust-web3) crate.

```

```
