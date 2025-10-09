//! Cryptographic operations for signature verification.
//!
//! This module provides cryptographic primitives for:
//! - Hashing (SHA-512Half, RIPEMD-160)
//! - Signature verification (ECDSA secp256k1, Ed25519)
//! - Address derivation from public keys

pub mod hash;
pub mod verify;

pub use hash::{account_id_from_pubkey, sha512half};
pub use verify::verify_signature;
