# Web3

_NOTE: not for use in production_

This Web3 driver is a learning aid for understanding how real Web3 drivers interact with Ethereum.

This crate interacts with the Hardhat chain in the [contracts](../contracts) directory.

## Create a Web3 Instance

```rust
let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
```

## Accounts

### Get All Accounts

```rust

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let all_accounts = web3.get_all_accounts().await;
```

### Get Account Balance

```rust
use web3::account::{get_all_accounts, get_balance};

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let account = web3.get_all_accounts().await.unwrap()[0].clone();
let balance = web3.get_balance(account).await;
```

### Get Account Balance from a Block

```rust
use types::block::BlockNumber;
use web3::account::{get_all_accounts, get_balance_by_block};

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let block = BlockNumber(0.into());
let account = web3.get_all_accounts().await.unwrap()[0];
let balance = web3.get_balance_by_block(account, Some(block)).await;
```

## Blocks

### Get Current Block Number

```rust
use web3::block::get_block_number;

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let block_number = web3.get_block_number()).await;
```

### Retrieve a Block

```rust
use web3::block::get_block;

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let block_number = U64::from(0);
let block = web3.get_block(block_number)).await;
```

## Contracts

### Deploy a Contract

```rust
use web3::contract::deploy;

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let account = web3.get_all_accounts().await.unwrap()[0];
let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let tx_hash = web3.deploy(account, &contract).await;
```

### Get Contract Code

```rust
use web3::contract::{code, deploy};

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let tx_hash = web3.deploy(account, &contract).await.unwrap();
let receipt = web3.transaction_receipt(tx_hash).await.unwrap();
let code = web3.code(receipt.contract_address.unwrap(), None).await;
```

## Transactions

### Send a Transaction

```rust
use types::transaction::TransactionRequest;

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let from = web3.get_all_accounts().await.unwrap()[0];
let to = web3.get_all_accounts().await.unwrap()[1];
let gas = U256::from(1_000_000);
let gas_price = U256::from(1);
let data = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let transaction_request = TransactionRequest {
    from: None,
    to: Some(to),
    value: None,
    gas,
    gas_price,
    data: Some(data.into()),
};
let tx_hash = web3.send(transaction_request).await;
```

### Get a Transaction Receipt

```rust
use types::transaction::TransactionRequest;

let web3 = web3::Web3::new("http://127.0.0.1:8545").unwrap();
let from = web3.get_all_accounts().await.unwrap()[0];
let to = web3.get_all_accounts().await.unwrap()[1];
let gas = U256::from(1_000_000);
let gas_price = U256::from(1);
let data = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let transaction_request = TransactionRequest {
    from: None,
    to: Some(to),
    value: None,
    gas,
    gas_price,
    data: Some(data.into()),
    };
let tx_hash = web3.send(transaction_request).await;
let receipt = web3.transaction_receipt(tx_hash).await;
```

## Other Work

For a full-blown crate that you can use in production, check out the official [Web3](https://github.com/tomusdrw/rust-web3) crate.

```

```
