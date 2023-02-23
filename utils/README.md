# Crypto

Functions for generating keys, hashing data, signing and verifying.

## Generate a private/public keypair


```rust
use crypto::keypair;

let (private_key, public_key) = keypair();
```

## Convert a public key into an address using the last 20 bytes of the Keccak256 hash

```rust
use crypto::{keypair, public_key_address};

let (private_key, public_key) = keypair();
let address = public_key_address(&public_key);
```

## Convert a private key into an address using the last 20 bytes of the Keccak256 hash

```rust
use crypto::{keypair, private_key_address};

let (private_key, public_key) = keypair();
let address = private_key_address(&private_key);
```

## Create a Keccak256 hash

```rust
use crypto::keccak256;

let message = b"The message";
let hashed = keccak256(message);
assert_eq!(hashed, [
    174, 253, 38, 204, 75, 207, 36, 167, 252, 109, 46, 248, 163, 40, 95, 14, 14, 198,
    197, 2, 119, 153, 141, 102, 195, 214, 250, 111, 247, 123, 45, 64
]);
```

## Sign a message with a private key

```rust
use crypto::{keypair, sign};

let (private_key, public_key) = keypair();
let message = b"The message";
let signature = sign(message, &private_key);
```

## Sign a recoverable message with a private key

```rust
use crypto::{keypair, sign_recovery};

let (private_key, public_key) = keypair();
let message = b"The message";
let signature = sign_recovery(message, &private_key);
```

## Verify that a message was signed using a public key

```rust
use crypto::{keypair, sign, sign_recovery, verify};

let (private_key, public_key) = keypair();
let message = b"The message";

let signature = sign(message, &private_key);
let serialized_signature = signature.serialize_compact();
let verified = verify(message, &serialized_signature, &public_key);
assert!(verified);

let signature = sign_recovery(message, &private_key);
let (_, serialized_signature) = signature.serialize_compact();
let verified = verify(message, &serialized_signature, &public_key);
assert!(verified);
```

## Recover a public key using a recoverable signature and signed message

```rust
use crypto::{keypair, recover_public_key, sign_recovery};

let (private_key, public_key) = keypair();
let message = b"The message";
let signature = sign_recovery(message, &private_key);
let (recovery_id, serialized_signature) = signature.serialize_compact();
let recovered_public_key = recover_public_key(message, &serialized_signature, recovery_id.to_i32());
assert_eq!(recovered_public_key, public_key);
```

## Recover the address of the public key using a recoverable signature and signed message

```rust
use crypto::{keypair, public_key_address, recover_address, sign_recovery};

let (private_key, public_key) = keypair();
let message = b"The message";
let signature = sign_recovery(message, &private_key);
let (recovery_id, serialized_signature) = signature.serialize_compact();
let recover_address = recover_address(message, &serialized_signature, recovery_id.to_i32());
assert_eq!(recover_address, public_key_address(&public_key));
```