use ethereum_types::{Address, Secret, U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub nonce: U256,
    pub hash: Secret,
    pub gas: U256,
    pub gas_price: U256,
    pub data: Option<Vec<u8>>,
}

// impl Transaction {
//     pub fn sign(&self, address: Address, data: Vec<u8>Vec<u8>)  {
//         let address = helpers::serialize(&address);
//         let data = helpers::serialize(&data);
//         CallFuture::new(self.transport.execute("eth_sign", vec![address, data]))
//     }
//     pub fn send(&self) {
//         let tx = helpers::serialize(&tx);
//         CallFuture::new(self.transport.execute("eth_sendTransaction", vec![tx]))
//     }
// }
