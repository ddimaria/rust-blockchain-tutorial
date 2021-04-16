# Types

Shared Ethereum types to be used by other members of this workspace.

## Accounts

In Ethereum, Accounts are just addresses.
Accounts can have ETH and tokens, and can send transactions to the chain.
A deployed contract is also an account.
Accounts can also interact with deployed contracts.

see https://ethereum.org/en/developers/docs/accounts/

## Blocks

Blocks are a fundamental aspect of the Ethereum blockchain.
A block can consist of many transactions.
Each block contains a hash of the parent block, which links blocks together.

A sample block from the chain:

```json
{
  "difficulty": String("0x0"),
  "extraData": String("0x"),
  "gasLimit": String("0x6691b7"),
  "gasUsed": String("0x0"),
  "hash": String(
    "0x7a2b18ecb9565eaa511601130d8108886b5d9cb14c6f9662c1e661bbfc73523e"
  ),
  "logsBloom": String(
    "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000"
  ),
  "miner": String("0x0000000000000000000000000000000000000000"),
  "mixHash": String(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  ),
  "nonce": String("0x0000000000000000"),
  "number": String("0x0"),
  "parentHash": String(
    "0x0000000000000000000000000000000000000000000000000000000000000000"
  ),
  "receiptsRoot": String(
    "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
  ),
  "sha3Uncles": String(
    "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
  ),
  "size": String("0x3e8"),
  "stateRoot": String(
    "0xd5b2d8fdfe99430dcdaa397d252d0cae3a1457c414999fbba318ba90ec0ed56b"
  ),
  "timestamp": String("0x60367687"),
  "totalDifficulty": String("0x0"),
  "transactions": Array([]),
  "transactionsRoot": String(
    "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
  ),
  "uncles": Array([])
}
```

see https://ethereum.org/en/developers/docs/blocks/
