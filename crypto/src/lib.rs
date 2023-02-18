use ethereum_types::{Address, H160, H256, U256};
use lazy_static::lazy_static;
pub use rlp::{Encodable, RlpStream};
pub use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId, Signature as EcdsaSignature},
    generate_keypair, rand, All, Message, PublicKey, Secp256k1, SecretKey,
};
use sha3::{Digest, Keccak256};
use types::bytes::Bytes;

lazy_static! {
    pub(crate) static ref CONTEXT: Secp256k1<All> = Secp256k1::new();
}

pub struct Signature {
    pub v: u64,
    pub r: H256,
    pub s: H256,
}

impl From<RecoverableSignature> for Signature {
    fn from(value: RecoverableSignature) -> Self {
        let (recovery_id, signature) = value.serialize_compact();

        let v = recovery_id.to_i32() as u64;
        let r = H256::from_slice(&signature[..32]);
        let s = H256::from_slice(&signature[32..]);

        Signature { v, r, s }
    }
}

// TODO(ddimaria): handle unwrap
impl Into<Bytes> for Signature {
    fn into(self) -> Bytes {
        Bytes({
            let mut bytes = Vec::with_capacity(65);
            bytes.extend_from_slice(self.r.as_bytes());
            bytes.extend_from_slice(self.s.as_bytes());
            bytes.push(self.v.try_into().unwrap());
            bytes
        })
    }
}

/// Generate a private/public keypair
///
/// ```rust
/// use crypto::keypair;
///
/// let (private_key, public_key) = keypair();
/// ```
pub fn keypair() -> (SecretKey, PublicKey) {
    generate_keypair(&mut rand::thread_rng())
}

/// Convert a public key into an address using the last 20 bytes of the Keccak256 hash
///
/// ```rust
/// use crypto::{keypair, public_key_address};
///
/// let (private_key, public_key) = keypair();
/// let address = public_key_address(&public_key);
/// ```
pub fn public_key_address(key: &PublicKey) -> H160 {
    let public_key = key.serialize_uncompressed();
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

/// Convert a private key into an address using the last 20 bytes of the Keccak256 hash
///
/// ```rust
/// use crypto::{keypair, private_key_address};
///
/// let (private_key, public_key) = keypair();
/// let address = private_key_address(&private_key);
/// ```
pub fn private_key_address(key: &SecretKey) -> H160 {
    let public_key = key.public_key(&CONTEXT);

    public_key_address(&public_key)
}

/// Create a Keccak256 hash
///
/// ```rust
/// use crypto::keccak256;
///
/// let message = b"The message";
/// let hashed = keccak256(message);
/// assert_eq!(hashed, [
///     174, 253, 38, 204, 75, 207, 36, 167, 252, 109, 46, 248, 163, 40, 95, 14, 14, 198,
///     197, 2, 119, 153, 141, 102, 195, 214, 250, 111, 247, 123, 45, 64
/// ]);
/// ```
pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

/// Sign a message with a private key
///
/// ```rust
/// use crypto::{keypair, sign};
///
/// let (private_key, public_key) = keypair();
/// let message = b"The message";
/// let signature = sign(message, &private_key);
/// ```
pub fn sign(message: &[u8], key: &SecretKey) -> EcdsaSignature {
    let message = hash_message(message);
    CONTEXT.sign_ecdsa(&message, key)
}

/// Sign a recoverable message with a private key
///
/// ```rust
/// use crypto::{keypair, sign_recovery};
///
/// let (private_key, public_key) = keypair();
/// let message = b"The message";
/// let signature = sign_recovery(message, &private_key);
/// ```
pub fn sign_recovery(message: &[u8], key: &SecretKey) -> RecoverableSignature {
    let message = hash_message(message);
    CONTEXT.sign_ecdsa_recoverable(&message, key)
}

/// Verify that a message was signed using a public key
///
/// ```rust
/// use crypto::{keypair, sign, sign_recovery, verify};
///
/// let (private_key, public_key) = keypair();
/// let message = b"The message";
///
/// let signature = sign(message, &private_key);
/// let serialized_signature = signature.serialize_compact();
/// let verified = verify(message, &serialized_signature, &public_key);
/// assert!(verified);
///
/// let signature = sign_recovery(message, &private_key);
/// let (_, serialized_signature) = signature.serialize_compact();
/// let verified = verify(message, &serialized_signature, &public_key);
/// assert!(verified);
/// ```
pub fn verify(message: &[u8], signature: &[u8], key: &PublicKey) -> bool {
    let message = hash_message(message);
    let signature = EcdsaSignature::from_compact(signature).unwrap();
    CONTEXT.verify_ecdsa(&message, &signature, key).is_ok()
}

/// Recover a public key using a recoverable signature and signed message
///
/// ```rust
/// use crypto::{keypair, recover_public_key, sign_recovery};
///
/// let (private_key, public_key) = keypair();
/// let message = b"The message";
/// let signature = sign_recovery(message, &private_key);
/// let (recovery_id, serialized_signature) = signature.serialize_compact();
/// let recovered_public_key = recover_public_key(message, &serialized_signature, recovery_id.to_i32());
/// assert_eq!(recovered_public_key, public_key);
/// ```
pub fn recover_public_key(message: &[u8], signature: &[u8], recovery_id: i32) -> PublicKey {
    let message = hash_message(message);
    let recovery_id = RecoveryId::from_i32(recovery_id).unwrap();
    let signature = RecoverableSignature::from_compact(signature, recovery_id).unwrap();

    CONTEXT.recover_ecdsa(&message, &signature).unwrap()
}

/// Recover the address of the public key using a recoverable signature and signed message
///
/// ```rust
/// use crypto::{keypair, public_key_address, recover_address, sign_recovery};
///
/// let (private_key, public_key) = keypair();
/// let message = b"The message";
/// let signature = sign_recovery(message, &private_key);
/// let (recovery_id, serialized_signature) = signature.serialize_compact();
/// let recover_address = recover_address(message, &serialized_signature, recovery_id.to_i32());
/// assert_eq!(recover_address, public_key_address(&public_key));
/// ```
pub fn recover_address(message: &[u8], signature: &[u8], recovery_id: i32) -> Address {
    let public_key = recover_public_key(message, signature, recovery_id);
    public_key_address(&public_key)
}

// Helper function to hash bytes and convert to a Message
pub fn hash_message(message: &[u8]) -> Message {
    let hashed = keccak256(message);
    Message::from_slice(&hashed).unwrap()
}

/// Encode items in a RlpStream
///
/// The RlP crate panics if stream.out() is invoked when the stream hasn't
/// consumed all list items (`list_size`).
pub fn rlp_encode<T: Encodable>(items: Vec<T>, signature: Option<&Signature>) -> RlpStream {
    let mut stream = RlpStream::new();

    let list_size = if signature.is_some() { 0 } else { 6 };
    stream.begin_list(list_size);

    items.iter().for_each(|item| {
        stream.append(item);
    });

    if let Some(signature) = signature {
        stream.append(&signature.v);
        stream.append(&U256::from_big_endian(signature.r.as_bytes()));
        stream.append(&U256::from_big_endian(signature.s.as_bytes()));
    }

    stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_and_public_key_addresses_match() {
        let (secret_key, public_key) = keypair();
        let private_key_address = private_key_address(&secret_key);
        let public_key_address = public_key_address(&public_key);
        assert_eq!(private_key_address, public_key_address);
    }

    #[test]
    fn it_hashes() {
        let message = b"The message";
        let hashed = keccak256(message);
        assert_eq!(
            hashed,
            [
                174, 253, 38, 204, 75, 207, 36, 167, 252, 109, 46, 248, 163, 40, 95, 14, 14, 198,
                197, 2, 119, 153, 141, 102, 195, 214, 250, 111, 247, 123, 45, 64
            ]
        );
    }

    #[test]
    fn it_recovers() {
        let (secret_key, public_key) = keypair();
        let message = b"The message";
        let signature = sign_recovery(message, &secret_key);
        let (recovery_id, serialized_signature) = signature.serialize_compact();
        let recovered_public_key =
            recover_public_key(message, &serialized_signature, recovery_id.to_i32());

        assert_eq!(recovered_public_key, public_key);

        let recovered_address =
            recover_address(message, &serialized_signature, recovery_id.to_i32());
        assert_eq!(recovered_address, public_key_address(&public_key));
    }

    #[test]
    fn it_verifies() {
        let (secret_key, public_key) = keypair();
        let message = b"The message";

        let signature = sign(message, &secret_key);
        let serialized_signature = signature.serialize_compact();
        let verified = verify(message, &serialized_signature, &public_key);
        assert!(verified);

        let signature = sign_recovery(message, &secret_key);
        let (_, serialized_signature) = signature.serialize_compact();
        let verified = verify(message, &serialized_signature, &public_key);
        assert!(verified);
    }

    #[test]
    fn it_rlp_encodes() {
        let items = vec!["a", "b", "c", "d", "e", "f"];
        let stream = rlp_encode(items, None);

        assert_eq!(stream.out().to_vec(), b"\xc6abcdef".to_vec());
    }
}
