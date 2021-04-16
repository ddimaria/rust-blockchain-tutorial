//! # Accounts
//!
//! In Ethereum, Accounts are just addresses.
//! Accounts can have ETH and tokens, and can send transactions to the chain.
//! A deployed contract is also an account.
//! Accounts can also interact with deployed contracts.
//!
//! see https://ethereum.org/en/developers/docs/accounts/

////////////////////////////////////////////////////////////////////////////////

use ethereum_types::Address;

pub type Account = Address;
