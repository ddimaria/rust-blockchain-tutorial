# Types

Shared Ethereum types to be used by other members of this workspace.

## Accounts

https://ethereum.org/en/developers/docs/accounts/

In Ethereum, Accounts are just addresses.
Accounts can have ETH and tokens, and can send transactions to the chain.
A deployed contract is also an account.
Accounts can also interact with deployed contracts.

## Blocks

https://ethereum.org/en/developers/docs/blocks/

Blocks are a fundamental aspect of the Ethereum blockchain.
A block can consist of many transactions.
Each block contains a hash of the parent block, which links blocks together.

A sample block from the chain:

```json
{
  "difficulty": "0x0",
  "extraData": "0x",
  "gasLimit": "0x6691b7",
  "gasUsed": "0x0",
  "hash": "0x7a2b18ecb9565eaa511601130d8108886b5d9cb14c6f9662c1e661bbfc73523e",
  "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
  "miner": "0x0000000000000000000000000000000000000000",
  "mixHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "nonce": "0x0000000000000000",
  "number": "0x0",
  "parentHash": "0x0000000000000000000000000000000000000000000000000000000000000000",
  "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
  "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
  "size": "0x3e8",
  "stateRoot": "0xd5b2d8fdfe99430dcdaa397d252d0cae3a1457c414999fbba318ba90ec0ed56b",
  "timestamp": "0x60367687",
  "totalDifficulty": "0x0",
  "transactions": [],
  "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
  "uncles": []
}
```

# Transactions

https://ethereum.org/en/developers/docs/transactions/

Accounts send transactions to the blockchain.
Within the blockchain, transactions are cryptographically signed.
Transactions live within blocks.
