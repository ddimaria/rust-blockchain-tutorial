use ethereum_types::U64;
use std::fmt::{Display, LowerHex};

use crate::error::BlockChainError;

#[allow(non_snake_case)]
pub fn hex_to_U64(hex: String) -> Result<U64, BlockChainError> {
    U64::from_str_radix(&hex, 16).map_err(|e| BlockChainError::ParseError(e.to_string()))
}

pub fn to_hex<T>(num: T) -> String
where
    T: Display + LowerHex,
{
    format!("{:#x}", num)
}
