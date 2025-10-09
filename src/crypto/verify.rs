use ed25519_dalek::{Signature as EdSignature, VerifyingKey as EdVerifyingKey, Verifier as _};
use secp256k1::{ecdsa::Signature as EcdsaSignature, Message, PublicKey, Secp256k1};

/// Verifies Ed25519 signature
fn verify_ed25519(pubkey: &[u8], signature: &[u8], digest: &[u8; 32]) -> bool {
    if pubkey.len() != 33 || signature.len() != 64 {
        log::warn!(
            "Invalid Ed25519 sizes: pubkey={}, sig={} (expected: pubkey=33, sig=64)",
            pubkey.len(),
            signature.len()
        );
        return false;
    }

    log::debug!("Verifying Ed25519 signature");
    log::debug!("PublicKey: {}", hex::encode(pubkey));
    log::debug!("Signature: {}", hex::encode(signature));

    let mut pk32 = [0u8; 32];
    pk32.copy_from_slice(&pubkey[1..]);

    let vk = match EdVerifyingKey::from_bytes(&pk32) {
        Ok(vk) => {
            log::debug!("Ed25519 verifying key parsed successfully");
            vk
        }
        Err(e) => {
            log::error!("Failed to parse Ed25519 verifying key: {:?}", e);
            return false;
        }
    };

    let mut sig_array = [0u8; 64];
    sig_array.copy_from_slice(signature);
    let edsig = EdSignature::from_bytes(&sig_array);

    match vk.verify(digest, &edsig) {
        Ok(_) => {
            log::debug!("Ed25519 signature verification: VALID");
            true
        }
        Err(e) => {
            log::warn!("Ed25519 signature verification failed: {:?}", e);
            false
        }
    }
}

/// Verifies secp256k1 ECDSA signature
fn verify_secp256k1(pubkey: &[u8], signature: &[u8], digest: &[u8; 32]) -> bool {
    log::debug!("Verifying secp256k1 ECDSA signature");
    log::debug!("PublicKey hex: {}", hex::encode(pubkey));
    log::debug!("Signature hex: {}", hex::encode(signature));
    log::debug!("Digest hex: {}", hex::encode(digest));

    let secp = Secp256k1::new();

    let ecdsa_sig = match EcdsaSignature::from_der(signature) {
        Ok(sig) => {
            log::debug!("DER signature parsed successfully");
            sig
        }
        Err(e) => {
            log::error!("Failed to parse DER signature: {:?}", e);
            return false;
        }
    };

    let pub_key = match PublicKey::from_slice(pubkey) {
        Ok(pk) => {
            log::debug!("Public key parsed successfully");
            pk
        }
        Err(e) => {
            log::error!("Failed to parse public key: {:?}", e);
            return false;
        }
    };

    let message = match Message::from_digest_slice(digest) {
        Ok(msg) => {
            log::debug!("Message created from digest");
            msg
        }
        Err(e) => {
            log::error!("Failed to create message from digest: {:?}", e);
            return false;
        }
    };

    match secp.verify_ecdsa(&message, &ecdsa_sig, &pub_key) {
        Ok(_) => {
            log::debug!("secp256k1 signature verification: VALID");
            true
        }
        Err(e) => {
            log::warn!("secp256k1 signature verification failed: {:?}", e);
            false
        }
    }
}

/// Verifies the signature (automatically detects Ed25519 or secp256k1)
pub fn verify_signature(pubkey: &[u8], signature: &[u8], digest: &[u8; 32]) -> bool {
    if pubkey.is_empty() {
        log::warn!("Empty public key provided");
        return false;
    }

    // Ed25519 keys start with 0xED
    if pubkey[0] == 0xED {
        log::info!("Detected Ed25519 signature algorithm");
        verify_ed25519(pubkey, signature, digest)
    } else {
        log::info!("Detected secp256k1 ECDSA signature algorithm");
        verify_secp256k1(pubkey, signature, digest)
    }
}
