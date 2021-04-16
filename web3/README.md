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
let account = get_all_accounts().await.unwrap()[0].clone();
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

## Other Work

For a full-blown crate that you can use in production, check out the official [Web3](https://github.com/tomusdrw/rust-web3) crate.

```

```
