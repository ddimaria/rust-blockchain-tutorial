# Web3

_NOTE: not for use in production_

This Web3 driver is a learning aid for understanding how real Web3 drivers interact with Ethereum.

This crate interacts with the Hardhat chain in the [contracts](../contracts) directory.

## Create a Web3 Instance

```rust
let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
```

## Accounts

### Get All Accounts

```rust

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let all_accounts = web3.get_all_accounts().await;
```

##### Response

```rust
Ok(
    [
      0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266,
      0x70997970c51812dc3a010c7d01b50e0d17dc79c8,
      0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc
    ]
)
```

### Get Account Balance

```rust
use web3::account::{get_all_accounts, get_balance};

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let account = web3.get_all_accounts().await?[0].clone();
let balance = web3.get_balance(account).await;
```

##### Response

```rust
Ok(9999870002304000000000)
```

### Get Account Balance from a Block

```rust
use types::block::BlockNumber;
use web3::account::{get_all_accounts, get_balance_by_block};

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let block = BlockNumber(0.into());
let account = web3.get_all_accounts().await?[0];
let balance = web3.get_balance_by_block(account, Some(block)).await;
```

##### Response

```rust
Ok(9999870002304000000000)
```

## Blocks

### Get Current Block Number

```rust
use web3::block::get_block_number;

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let block_number = web3.get_block_number()).await;
```

##### Response

```rust
Ok(BlockNumber(42))
```

### Retrieve a Block

```rust
use web3::block::get_block;

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let block_number = U64::from(42);
let block = web3.get_block(block_number)).await;
```

##### Response

```rust
Ok(
    Block {
      hash: 0x43bf92a90221d68aa5bfd1ad65b3d3ace2c97b5783a358b30ac3304820310751,
      parent_hash: 0x41d5898f86a439d5ec116965cdbc090f1a2bdf6b76465a8832455b9de0569898,
      sha3_uncles: 0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347,
      miner: 0xc014ba5ec014ba5ec014ba5ec014ba5ec014ba5e,
      state_root: 0x82e2ebb959aa35579c6224777a3f563717ed006afd4a3ad4bd14d28d92efbfeb,
      transactions_root: 0xfbeeb53cdce587225bea5716e91b2f4c2d034b812505ae8ac400e80f4161ae10,
      receipts_root: 0x969a8cd0591bceb012fea9973b866ea96b5aa14fc8d9cae7ba1757b6fba34f95,
      number: 38,
      gas_used: 0,
      gas_limit: 9500000,
      logs_bloom: Some(
          0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
      ),
      timestamp: 1618659703,
      difficulty: 131264,
      total_difficulty: Some(
          4984321,
      ),
      seal_fields: None,
      uncles: [],
      transactions: [
          Transaction {
              from: 0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266,
              to: 0x70997970c51812dc3a010c7d01b50e0d17dc79c8,
              value: 0,
              nonce: 37,
              hash: 0x72d0f35eec01aad910af8a923bf256c7a198aa6ee117221bed6078f5fb8cc760,
              gas: 1000000,
              gas_price: 8000000000,
              data: None,
          },
      ],
      size: Some(
          26027,
      ),
      mix_hash: Some(
          0x0000000000000000000000000000000000000000000000000000000000000000,
      ),
      nonce: Some(
          0x0000000000000042,
      ),
  }
)
```

## Contracts

### Deploy a Contract

```rust
use web3::contract::deploy;

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let account = web3.get_all_accounts().await?[0];
let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let tx_hash = web3.deploy(account, &contract).await;
```

#### Response

```rust
Ok(0x5581416b14f1cffae922ae5507528e8e6d3066c06bd8e8553f90cd2f45c21cc0)
```

### Get Contract Code

```rust
use web3::contract::{code, deploy};

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let contract = include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json").to_vec();
let tx_hash = web3.deploy(account, &contract).await?;
let receipt = web3.transaction_receipt(tx_hash).await?;
let code = web3.code(receipt.contract_address?, None).await;
```

#### Response

```rust
TBD
```

## Transactions

### Send a Transaction

```rust
use types::transaction::TransactionRequest;

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let from = web3.get_all_accounts().await?[0];
let to = web3.get_all_accounts().await?[1];
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

#### Response

```rust
Ok(0x5581416b14f1cffae922ae5507528e8e6d3066c06bd8e8553f90cd2f45c21cc0)
```

### Get a Transaction Receipt

```rust
use types::transaction::TransactionRequest;

let web3 = web3::Web3::new("http://127.0.0.1:8545")?;
let from = web3.get_all_accounts().await?[0];
let to = web3.get_all_accounts().await?[1];
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

#### Response

```rust
TransactionReceipt {
    transaction_hash: 0xb439110d466b9fb8d92181b10c10d5b555d8e24602edd7879e74607d5ba286ed,
    transaction_index: "0x0",
    block_hash: Some(
        0x350549330f25373e136bfc15abcbc883fb33cb6c8e6b4605b53cfe6c254557a2,
    ),
    block_number: Some(
        BlockNumber(
            12,
        ),
    ),
    cumulative_gas_used: 427624,
    gas_used: Some(
        427624,
    ),
    contract_address: None,
    logs: [],
    status: Some(
        1,
    ),
    root: None,
    logs_bloom: 0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000,
}
```

## Other Work

For a full-blown crate that you can use in production, check out the official [Web3](https://github.com/tomusdrw/rust-web3) crate.

```

```
