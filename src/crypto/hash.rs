use ripemd::Ripemd160;
use sha2::{Digest, Sha256, Sha512};

/// Calculates SHA-512 and returns the first half (32 bytes)
pub fn sha512half(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut half = [0u8; 32];
    half.copy_from_slice(&out[..32]);
    half
}

/// Derives the Account ID (20 bytes) from a public key
/// using SHA-256 followed by RIPEMD-160
pub fn account_id_from_pubkey(pubkey: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(pubkey);
    let ripemd = Ripemd160::digest(sha);
    let mut out = [0u8; 20];
    out.copy_from_slice(&ripemd[..20]);
    out
}
