# Chain

_NOTE: not for use in production_

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
        0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266,
        0x70997970c51812dc3a010c7d01b50e0d17dc79c8,
        0x3c44cdddb6a900fa2b585dd299e03d12fa4293bc
    ]
}
```

#### Get Account Balance

```shell
curl -X POST \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"2.0","id":"id","method":"eth_getBalance","params":[0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266]}' \
     http://127.0.0.1:8545
```

###### Response

```json
{
    "jsonrpc":"2.0",
    "id":"id",
    "result":[10]
}
```