# Chain

_NOTE: not for use in production_

This crate is a simplistic ethereum blockchain. The goal is to implement major features to fully integrate with the [web3](../web3) crate.

## Start a Chain Node

```shell
RUST_LOG=info cargo run
```

## API

### Accounts

#### Get All Accounts

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

#### Get Account Balance

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getBalance","params":["0xfbb55f17b2926063ae3fa5647c98eb1fac88c99e"]}' \
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":100
}
```

#### Get Account Balance

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getBalance","params":["0x1baa70d7b3b679db9103f0b539b689d9e5cbcb00"]}' \
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":"0x64"
}
```

#### Get Account Balance by Block

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getBalanceByBlock","params":["0x1baa70d7b3b679db9103f0b539b689d9e5cbcb00", "0x0"]}' \
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":"0x64"
}
```

#### Get Current Block Number

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_blockNumber","params":[]}' \
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":"0x2"
}
```

#### Get Block by Block Number

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getBlockByNumber","params":["0x2"]}' \
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":
    {
        "hash":"0x24949baafab48f0942a7f6d26395d3b029864659efe18c5901d74b9f7026b498",
        "nonce":"0x2f4b75fde48db7e9c7634202cd93bff74e308bbc2fb1415b904e345bef9e9730",
        "number":"0x2",
        "parentHash":"0x7edbeeee490aaef6bea4281ccb1e095b90d75d89b4ecc8ebf9a7f7ef12af8322",
        "transactions":[
            {
                "data":null,
                "from":"0x1baa70d7b3b679db9103f0b539b689d9e5cbcb00",
                "hash":"0xe7ea9384ee161202351d462669cd09713448f66492020e2102446b3720ffb6f2",
                "nonce":"0x0",
                "to":"0xe55e60dddb23f9878f9a879f1f65eb36c0620f3f",
                "value":"0x1"
            }
        ]
    }
}
```

#### Send a Transaction

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_sendTransaction","params":[{"data":"0x7b0a2020225f66a7","from":"0x23cb46b0482d691f2a6094ffc0c650c982a9ed34","gas":"0xf4240","gasPrice":"0x1","to":"0xc58f06989ceb6a80ade923ba358b85bb502a276b","value":"0x0"}]}' \
     http://127.0.0.1:8545
```

###### Response

Receive a transaction receipt hash on a successful acceptance of a transaction

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":"0x6754bbb902a98df555d58f2e56c662b3fe2727ca3ef2b0f531f840b0e8bf1416"
}
```

#### Get a Transaction Receipt

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getTransactionReceipt","params":["0x6754bbb902a98df555d58f2e56c662b3fe2727ca3ef2b0f531f840b0e8bf1416"]}' \
     http://127.0.0.1:8545
```

###### Response

Receive a transaction receipt of a process transaction

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":
    {
        "blockHash":"0x6754bbb902a98df555d58f2e56c662b3fe2727ca3ef2b0f531f840b0e8bf1416",
        "blockNumber":"0x0",
        "contractAddress":"0x1e10dce69fe64c2b9f6990c1f3f5f0418324b145",
        "transactionHash":"0x6754bbb902a98df555d58f2e56c662b3fe2727ca3ef2b0f531f840b0e8bf1416"
    }
}
```

#### Get a Contract's Code

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getCode","params":["0x1e10dce69fe64c2b9f6990c1f3f5f0418324b145", "latest"]}' \
     http://127.0.0.1:8545
```

###### Response

The contract's code

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":"0x7b0a2020225f66a7"
}
```