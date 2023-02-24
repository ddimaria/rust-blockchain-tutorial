use wit_bindgen_guest_rust::*;

wit_bindgen_guest_rust::generate!({path: "../erc20/erc20.wit", world: "erc20"});

struct Erc20 {}

export_contract!(Erc20);

impl erc20::Erc20 for Erc20 {
    fn mint(address: String, amount: u64) {
        println!("Hello {}", address);
    }
}
