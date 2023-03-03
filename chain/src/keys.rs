use crate::error::{ChainError, Result};
use ethereum_types::Address;
use lazy_static::lazy_static;
use std::fs::{create_dir, read, write};
use utils::{
    crypto::{keypair, public_key_address},
    PublicKey, SecretKey,
};

const PATH: &str = "./../.keys";
const PRIVATE_KEY_PATH: &str = "./../.keys/private.key";
const PUBLIC_KEY_PATH: &str = "./../.keys/public.key";

lazy_static! {
    pub(crate) static ref PRIVATE_KEY: SecretKey =
        get_private_key().expect("Could not retrieve the private key");
    pub(crate) static ref PUBLIC_KEY: PublicKey =
        get_public_key().expect("Could not retrieve the public key");
    pub(crate) static ref ADDRESS: Address = public_key_address(&PUBLIC_KEY);
}

pub(crate) fn add_keys() -> Result<()> {
    if let Err(e) = create_dir(PATH) {
        tracing::info!("Did not create key directory '{}' {}", PATH, e.to_string());
    } else {
        let (private_key, public_key) = keypair();

        write(PRIVATE_KEY_PATH, private_key.as_ref()).unwrap();
        write(PUBLIC_KEY_PATH, public_key.serialize()).unwrap();
    }

    Ok(())
}

pub(crate) fn get_private_key() -> Result<SecretKey> {
    let key = read(PRIVATE_KEY_PATH).expect("Could not read private key");
    SecretKey::from_slice(&key).map_err(|e| ChainError::InternalError(e.to_string()))
}

pub(crate) fn get_public_key() -> Result<PublicKey> {
    let key = read(PUBLIC_KEY_PATH).expect("Could not read public key");
    PublicKey::from_slice(&key).map_err(|e| ChainError::InternalError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_save_keys() {
        add_keys().unwrap();
    }

    #[test]
    fn it_retrieves_the_saved_private_key() {
        add_keys().unwrap();
        let key = get_private_key().unwrap();
        println!("{:?}", key);
    }

    #[test]
    fn it_retrieves_the_saved_public_key() {
        add_keys().unwrap();
        let key = get_public_key().unwrap();
        println!("{:?}", key);
    }
}
