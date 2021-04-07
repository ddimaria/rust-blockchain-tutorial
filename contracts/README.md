# Smart Contracts

This section is JavaScript-based, and leverages Hardhat.

## Installation

### Install Node and NPM

This project was built using Node version 14.

```shell
brew install node
```

### Download Dependencies

```shell
npm i
```

## Starting the Chain

To interact with the chain, you'll need to start it up:

```shell
npx hardhat node
```

This will run the chain on `http://127.0.0.1:8545`. At startup, auto-created accounts will be displayed, along with their private keys.

## Compiling Contracts

```shell
npx hardhat compile
```
