#![allow(unused)]
use std::collections::HashMap;

wit_bindgen::generate!("erc20");

pub struct Erc20;

pub struct State {
    name: String,
    symbol: String,
    balances: HashMap<String, u64>,
}

export_contract!(Erc20);

impl Contract for Erc20 {
    fn construct(name: String, symbol: String) {
        println!("name {}, symbol", symbol);
    }

    fn mint(account: String, amount: u64) {
        println!("account {}, amount", amount);
    }

    fn transfer(to: String, amount: u64) {
        println!("to {}, amount", amount);
    }
}
