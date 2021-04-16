//! # Bytes
//!
//! Bytes is a wrapper type for Vec<u8>.
//! They are wrapped to automatically convert the bytes into hex
//! when serialiazing, and from hex back to bytes when deserializing.

////////////////////////////////////////////////////////////////////////////////

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Bytes(pub Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for Bytes {
    fn from(data: T) -> Self {
        Bytes(data.into())
    }
}

// Convert Bytes to hex when serializing
impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let serialized = format!("0x{}", hex::encode(&self.0));
        serializer.serialize_str(&serialized)
    }
}

// Convert hex to bytes when deserializing
impl<'a> Deserialize<'a> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
    where
        D: Deserializer<'a>,
    {
        let raw: &[u8] = Deserialize::deserialize(deserializer)?;

        if raw.starts_with(b"0x") {
            let bytes =
                hex::decode(&raw[2..]).map_err(|e| Error::custom(format!("Invalid hex: {}", e)))?;
            Ok(Bytes(bytes))
        } else {
            Err(Error::custom(format!("Missing 0x prefix")))
        }
    }
}
