use ethereum_types::Address;
use ethereum_types::U256;
use types::bytes::Bytes;
use types::transaction::TransactionRequest;

use crate::error::Result;
use crate::transaction::send_transaction;

pub async fn deploy(owner: Address, abi: &[u8]) -> Result<U256> {
    let gas = U256::from(1_000_000);
    let gas_price = U256::from(1_000_000);
    let data: Bytes = abi.into();
    let transaction_request = TransactionRequest {
        from: None,
        to: Some(owner),
        value: None,
        gas,
        gas_price,
        data: Some(data),
    };

    send_transaction(transaction_request).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::get_all_accounts;
    use crate::helpers::tests::get_contract;

    #[tokio::test]
    async fn it_deploys_a_contract() {
        let to = get_all_accounts().await.unwrap()[1].clone();
        let data = get_contract();
        let response = deploy(to, &data).await;
        assert!(response.is_ok());
    }
}
