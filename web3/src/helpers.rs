#[cfg(test)]
pub mod tests {
    pub fn get_contract() -> Vec<u8> {
        include_bytes!("./../../contracts/artifacts/contracts/ERC20.sol/RustCoinToken.json")
            .to_vec()
    }
}
