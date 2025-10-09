use super::recover::recover_pubkey_from_signature;
use crate::crypto::{account_id_from_pubkey, sha512half};
use crate::types::VerificationResult;
use hex::FromHex;
use secp256k1::{ecdsa::Signature as EcdsaSignature, Message, PublicKey, Secp256k1};

/// Converts DER signature to compact format (r, s)
fn der_to_compact(der_sig: &[u8]) -> Option<[u8; 64]> {
    match EcdsaSignature::from_der(der_sig) {
        Ok(sig) => Some(sig.serialize_compact()),
        Err(_) => None,
    }
}

/// Creates the message hash in the format Web3Auth expects
fn hash_challenge_for_web3auth(challenge: &str) -> [u8; 32] {
    // Web3Auth typically uses the challenge directly or with a prefix
    // Let's try SHA-512Half (XRPL standard)
    sha512half(challenge.as_bytes())
}

/// Verifies Web3Auth signature
pub fn verify_web3auth_signature(
    signature_hex: &str,
    challenge: &str,
    expected_address: &str,
) -> anyhow::Result<VerificationResult> {
    log::debug!("Web3Auth verification starting");
    log::debug!("Signature (DER): {}", signature_hex);
    log::debug!("Challenge: {}", challenge);
    log::debug!("Expected Address: {}", expected_address);

    // Parse signature DER
    let signature_der = Vec::from_hex(signature_hex)?;

    // Convert DER to compact format
    let signature_compact = der_to_compact(&signature_der)
        .ok_or_else(|| anyhow::anyhow!("Failed to parse DER signature"))?;

    log::debug!("DER signature converted to compact format");

    // Hash the challenge
    let message_hash = hash_challenge_for_web3auth(challenge);
    log::debug!("Message hash: {}", hex::encode(message_hash));

    // Recover possible public keys
    let pubkey_candidates = recover_pubkey_from_signature(&message_hash, &signature_compact);
    log::debug!("Found {} pubkey candidates", pubkey_candidates.len());

    // Try each candidate and see which one matches the address
    for (i, pubkey) in pubkey_candidates.iter().enumerate() {
        let account_id = account_id_from_pubkey(pubkey);
        let derived_address = ripple_address_codec::encode_account_id(&account_id);

        log::debug!("Candidate {}: {}", i, derived_address);

        if derived_address == expected_address {
            // Found the right pubkey! Now verify the signature
            let signature_valid = verify_with_pubkey(pubkey, &signature_der, &message_hash);

            log::info!("Address match found! Signature valid: {}", signature_valid);
            log::debug!("Matched address: {}", derived_address);

            return Ok(VerificationResult {
                address_valid: true,
                challenge_valid: true, // Challenge is what we signed
                signature_valid,
                derived_address,
                found_challenge: Some(challenge.to_string()),
            });
        }
    }

    // No matching pubkey found
    log::warn!("No pubkey candidate matched the expected address");

    Ok(VerificationResult {
        address_valid: false,
        challenge_valid: true,
        signature_valid: false,
        derived_address: String::new(),
        found_challenge: Some(challenge.to_string()),
    })
}

/// Verifies the signature using the recovered public key
fn verify_with_pubkey(pubkey: &[u8], signature_der: &[u8], message_hash: &[u8; 32]) -> bool {
    let secp = Secp256k1::new();

    let sig = match EcdsaSignature::from_der(signature_der) {
        Ok(s) => {
            log::debug!("DER signature parsed for verification");
            s
        }
        Err(e) => {
            log::error!("Failed to parse DER signature for verification: {:?}", e);
            return false;
        }
    };

    let pk = match PublicKey::from_slice(pubkey) {
        Ok(p) => {
            log::debug!("Public key parsed successfully");
            p
        }
        Err(e) => {
            log::error!("Failed to parse public key: {:?}", e);
            return false;
        }
    };

    let msg = match Message::from_digest_slice(message_hash) {
        Ok(m) => {
            log::debug!("Message created from digest");
            m
        }
        Err(e) => {
            log::error!("Failed to create message from digest: {:?}", e);
            return false;
        }
    };

    match secp.verify_ecdsa(&msg, &sig, &pk) {
        Ok(_) => {
            log::debug!("ECDSA signature verification: VALID");
            true
        }
        Err(e) => {
            log::warn!("ECDSA signature verification failed: {:?}", e);
            false
        }
    }
}
