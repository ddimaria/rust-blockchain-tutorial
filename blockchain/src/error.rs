#[derive(Debug)]
pub enum BlockChainError {
    ClientError(String),
    ParseError(String),
    RequestError(String),
    ResponseError(String),
}

pub type Result<T> = std::result::Result<T, BlockChainError>;

impl From<serde_json::Error> for BlockChainError {
    fn from(error: serde_json::Error) -> Self {
        BlockChainError::ParseError(error.to_string())
    }
}
