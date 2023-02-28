//! # Bytes
//!
//! Bytes is a wrapper type for Vec<u8>.
//! They are wrapped to automatically convert the bytes into hex
//! when serialiazing, and from hex back to bytes when deserializing.

////////////////////////////////////////////////////////////////////////////////

pub use bytes::Bytes;
