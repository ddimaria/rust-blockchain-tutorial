use async_jsonrpc_client::Params;
use ethereum_types::H256;
use serde_json::to_value;
use types::transaction::TransactionRequest;

use crate::error::Result;
use crate::request::send_rpc;

pub async fn send(transaction_request: TransactionRequest) -> Result<H256> {
    let transaction_request = to_value(&transaction_request)?;
    let params = Params::Array(vec![transaction_request]);
    let response = send_rpc("eth_sendTransaction", Some(params)).await?;
    let tx_hash: H256 = serde_json::from_value(response)?;

    Ok(tx_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::get_all_accounts;
    use crate::helpers::tests::get_contract;
    use ethereum_types::U256;

    #[tokio::test]
    async fn it_sends_a_transaction() {
        let to = get_all_accounts().await.unwrap()[1];
        let gas = U256::from(1_000_000);
        let gas_price = U256::from(1);
        let data = get_contract();
        let transaction_request = TransactionRequest {
            from: None,
            to: Some(to),
            value: None,
            gas,
            gas_price,
            data: Some(data.into()),
        };
        let response = send(transaction_request).await;
        assert!(response.is_ok());
    }
}
