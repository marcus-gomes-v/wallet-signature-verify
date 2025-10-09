use secp256k1::{ecdsa::RecoverableSignature, Message, Secp256k1};

/// Attempts to recover the public key from an ECDSA signature
/// Tries all 4 possible recovery IDs (0-3)
pub fn recover_pubkey_from_signature(
    message_hash: &[u8; 32],
    signature_compact: &[u8; 64],
) -> Vec<Vec<u8>> {
    log::debug!("Starting public key recovery from ECDSA signature");
    log::debug!("Message hash: {}", hex::encode(message_hash));
    log::debug!("Signature (compact): {}", hex::encode(signature_compact));

    let secp = Secp256k1::new();
    let mut candidates = Vec::new();

    let message = match Message::from_digest_slice(message_hash) {
        Ok(m) => {
            log::debug!("Message created from digest slice");
            m
        }
        Err(e) => {
            log::error!("Failed to create message from digest: {:?}", e);
            return candidates;
        }
    };

    // Try all 4 possible recovery IDs
    for recovery_id in 0..4 {
        log::debug!("Trying recovery ID: {}", recovery_id);

        if let Ok(rec_id) = secp256k1::ecdsa::RecoveryId::from_i32(recovery_id) {
            if let Ok(recoverable_sig) =
                RecoverableSignature::from_compact(signature_compact, rec_id)
            {
                if let Ok(pubkey) = secp.recover_ecdsa(&message, &recoverable_sig) {
                    let pubkey_bytes = pubkey.serialize().to_vec();
                    log::debug!("Successfully recovered pubkey with recovery ID {}: {}",
                        recovery_id,
                        hex::encode(&pubkey_bytes)
                    );
                    candidates.push(pubkey_bytes);
                }
            }
        }
    }

    log::debug!("Public key recovery complete. Found {} candidates", candidates.len());
    candidates
}
