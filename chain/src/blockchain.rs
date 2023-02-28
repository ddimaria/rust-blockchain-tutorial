//! # Chain
//!
//! The container for the blockchain

////////////////////////////////////////////////////////////////////////////////

use std::collections::VecDeque;
use std::sync::Arc;

use crate::account::AccountStorage;
use crate::error::{ChainError, Result};
use crate::storage::Storage;
use crate::transaction::TransactionStorage;
use crate::world_state::WorldState;
use ethereum_types::{H160, H256, U256, U64};
use tokio::sync::Mutex;
use types::account::Account;
use types::block::{Block, BlockNumber};
use types::bytes::Bytes;
use types::transaction::{
    SignedTransaction, Transaction, TransactionKind, TransactionReceipt, TransactionRequest,
};
use utils::{Digest, Keccak256};

// TODO(ddimaria): store blocks in a patricia merkle trie
#[derive(Debug)]
pub(crate) struct BlockChain {
    pub(crate) accounts: AccountStorage,
    pub(crate) blocks: Vec<Block>,
    pub(crate) transactions: Arc<Mutex<TransactionStorage>>,
    pub(crate) world_state: WorldState,
}

impl BlockChain {
    pub(crate) fn new(storage: Arc<Storage>) -> Result<Self> {
        Ok(Self {
            accounts: AccountStorage::new(storage),
            blocks: vec![Block::genesis()?],
            transactions: Arc::new(Mutex::new(TransactionStorage::new())),
            world_state: WorldState::new(),
        })
    }

    pub(crate) fn get_current_block(&self) -> Result<Block> {
        let block = self
            .blocks
            .last()
            .ok_or_else(|| ChainError::BlockNotFound("current block".into()))?;

        Ok(block.to_owned())
    }

    pub(crate) fn get_block_by_number(&self, block_number: U64) -> Result<Block> {
        let index = (block_number - 1).as_usize();
        let block = self
            .blocks
            .get(index)
            .ok_or_else(|| ChainError::BlockNotFound("current block".into()))?;

        Ok(block.to_owned())
    }

    pub(crate) fn parse_block_number(&self, block_number: &str) -> Result<BlockNumber> {
        if block_number == "latest" {
            Ok(BlockNumber(self.get_current_block()?.number))
        } else {
            Ok(block_number
                .try_into()
                .map_err(|_| ChainError::InvalidBlockNumber(block_number.into()))?)
        }
    }

    pub(crate) fn new_block(&mut self, transactions: Vec<Transaction>) -> Result<Block> {
        // TODO(ddimaria): make this an atomic operation
        let current_block = self.get_current_block()?;
        let number = current_block.number + 1_u64;
        let parent_hash = current_block
            .hash
            .ok_or_else(|| ChainError::MissingHash(current_block.number.to_string()))?;
        let serialized = bincode::serialize(&(number, parent_hash, &transactions))?;
        let nonce = Keccak256::digest(serialized);

        let block = Block::new(
            number,
            H256::from(nonce.as_ref()),
            parent_hash,
            transactions,
        )?;

        self.blocks.push(block);

        self.get_block_by_number(number)
    }

    // TODO(ddimaria): remove auto nonce incrementing and rely on clients to increment
    pub(crate) async fn send_transaction(
        &mut self,
        transaction_request: TransactionRequest,
    ) -> Result<H256> {
        let value = transaction_request.value.unwrap_or(U256::zero());
        let from = transaction_request.from.unwrap_or(H160::zero());
        let to = transaction_request.to;
        let nonce = self.accounts.increment_nonce(&from)?.into();

        let transaction: Transaction =
            Transaction::new(from, to, value, nonce, transaction_request.data)?;
        let hash = transaction.transaction_hash()?;

        // add to the transaction mempool
        self.transactions.lock().await.send_transaction(transaction);

        Ok(hash)
    }

    pub(crate) async fn send_raw_transaction(&mut self, transaction: Bytes) -> Result<H256> {
        let signed_transaction: SignedTransaction = bincode::deserialize(&transaction)?;

        let verified = Transaction::verify(signed_transaction.clone())
            .map_err(|e| ChainError::TransactionNotVerified(e.to_string()))?;

        if !verified {
            return Err(ChainError::TransactionNotVerified(
                signed_transaction.transaction_hash.to_string(),
            ));
        }

        let transaction: Transaction = signed_transaction.try_into()?;

        self.send_transaction(transaction.into()).await
    }

    // TODO(ddimaria): handle TransactionKind::ContractExecution
    pub(crate) async fn process_transactions(&mut self) -> Result<()> {
        // Bulk drain the current queue to fit into the new block
        // This is not safe as we lose transactions if a panic occurs
        // or if the program is halted
        let transactions = self
            .transactions
            .lock()
            .await
            .mempool
            .drain(0..)
            .collect::<VecDeque<_>>();

        if !transactions.is_empty() {
            let mut receipts: Vec<TransactionReceipt> = vec![];
            let mut processed: Vec<Transaction> = vec![];

            tracing::info!(
                "Processing {} transactions for new block",
                transactions.len()
            );

            for transaction in transactions.into_iter() {
                tracing::info!("Processing Transaction {:?}", transaction.hash);

                let transaction_hash = transaction.transaction_hash()?;
                let value = transaction.value;
                let mut contract_address: Option<Account> = None;

                // create the `to` account if it doesn't exist
                if let Some(to) = transaction.to {
                    self.accounts.add_empty_account(&to)?;
                }

                // TODO(ddimaria): remove this copy
                let kind = transaction.to_owned().kind()?;

                match kind {
                    TransactionKind::Regular(from, to) => self.accounts.transfer(&from, &to, value),
                    TransactionKind::ContractDeployment(from, data) => {
                        contract_address = self.accounts.add_contract_account(&from, data).ok();
                        Ok(())
                    }
                    TransactionKind::ContractExecution(_from, _to, _data) => {
                        unimplemented!()
                    }
                }?;

                let transaction_receipt = TransactionReceipt {
                    block_hash: None,
                    block_number: None,
                    contract_address,
                    transaction_hash,
                };

                receipts.push(transaction_receipt);
                processed.push(transaction);
            }

            let num_processed = processed.len();
            let block = self.new_block(processed)?;

            tracing::info!(
                "Created block {} with {} transactions",
                block.number,
                num_processed
            );

            // now add the block number and hash to the receipts
            for mut receipt in receipts.into_iter() {
                receipt.block_number = Some(BlockNumber(block.number));
                receipt.block_hash = block.hash;

                self.transactions
                    .clone()
                    .lock()
                    .await
                    .receipts
                    .insert(receipt.transaction_hash, receipt);
            }

            // update world state
            let state_trie = self.accounts.hash_root()?;
            self.world_state.update_state_trie(state_trie);

            tracing::info!("World State: state_trie {:?}", state_trie);

            let storage = self.transactions.lock().await;

            tracing::info!(
                "Transaction storage: mempool {:?}, receipts {:?}",
                storage.mempool.len(),
                storage.receipts.len()
            );
        }

        Ok(())
    }

    pub(crate) async fn get_transaction_receipt(
        &mut self,
        transaction_hash: H256,
    ) -> Result<TransactionReceipt> {
        let transaction_receipt = self
            .transactions
            .lock()
            .await
            .get_transaction_receipt(&transaction_hash)?;

        Ok(transaction_receipt)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use utils::crypto::keypair;

    use super::*;
    use crate::helpers::tests::{setup, ACCOUNT_1, STORAGE};

    pub(crate) fn new_blockchain() -> BlockChain {
        BlockChain::new((*STORAGE).clone()).unwrap()
    }

    pub(crate) fn new_transaction(to: Account) -> Transaction {
        Transaction::new(*ACCOUNT_1, Some(to), U256::from(10), U256::zero(), None).unwrap()
    }

    pub(crate) async fn process_transactions(blockchain: Arc<Mutex<BlockChain>>) {
        blockchain
            .lock()
            .await
            .process_transactions()
            .await
            .unwrap();
    }

    pub(crate) async fn assert_receipt(blockchain: Arc<Mutex<BlockChain>>, transaction_hash: H256) {
        process_transactions(blockchain.clone()).await;

        let receipt = blockchain
            .lock()
            .await
            .transactions
            .lock()
            .await
            .get_transaction_receipt(&transaction_hash);

        assert!(receipt.is_ok());
    }

    pub(crate) async fn get_balance(blockchain: Arc<Mutex<BlockChain>>, account: &Account) -> U256 {
        blockchain
            .lock()
            .await
            .accounts
            .get_account(account)
            .unwrap()
            .balance
    }

    #[tokio::test]
    async fn creates_a_blockchain() {
        new_blockchain();
    }

    #[tokio::test]
    async fn creates_and_gets_a_block() {
        let mut blockchain = new_blockchain();
        let block_number = blockchain.get_current_block().unwrap().number;
        let response = blockchain.new_block(vec![new_transaction(Account::random())]);
        assert!(response.is_ok());

        let new_block_number = blockchain.get_current_block().unwrap().number;
        assert_eq!(new_block_number, block_number + 1);
    }

    #[tokio::test]
    async fn sends_a_transaction() {
        let (blockchain, _, _) = setup().await;
        let to = Account::random();
        let transaction = new_transaction(to);
        let transaction_hash = blockchain
            .lock()
            .await
            .send_transaction(transaction.into())
            .await
            .unwrap();

        assert_receipt(blockchain.clone(), transaction_hash).await;

        let balance = get_balance(blockchain, &to).await;
        assert_eq!(balance, U256::from(10));
    }

    #[tokio::test]
    async fn send_a_raw_transaction() {
        let (blockchain, _, _) = setup().await;
        let to = Account::random();
        let (secret_key, _) = keypair();
        let transaction = new_transaction(to);
        let signed_transaction = transaction.sign(secret_key).unwrap();
        let encoded = bincode::serialize(&signed_transaction).unwrap();
        let response = blockchain
            .lock()
            .await
            .send_raw_transaction(encoded.into())
            .await
            .unwrap();

        assert_receipt(blockchain.clone(), response).await;

        let balance = get_balance(blockchain, &to).await;
        assert_eq!(balance, U256::from(10));
    }
}
