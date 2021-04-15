#[derive(Debug)]
pub enum TypeError {
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, TypeError>;

impl From<serde_json::Error> for TypeError {
    fn from(error: serde_json::Error) -> Self {
        TypeError::ParseError(error.to_string())
    }
}
