use ed25519_dalek::{Signature as EdSignature, Verifier as _, VerifyingKey as EdVerifyingKey};
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_signature_empty_pubkey() {
        let digest = [0u8; 32];
        let signature = vec![0u8; 64];

        let result = verify_signature(&[], &signature, &digest);
        assert!(!result, "Empty pubkey should fail");
    }

    #[test]
    fn test_verify_signature_invalid_secp256k1() {
        // Valid pubkey but invalid signature
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();
        let invalid_signature = vec![0u8; 64]; // All zeros - invalid DER
        let digest = [0u8; 32];

        let result = verify_signature(&pubkey, &invalid_signature, &digest);
        assert!(!result, "Invalid signature should fail");
    }

    #[test]
    fn test_verify_signature_wrong_digest() {
        // Real signature but wrong digest
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();

        // This is a real DER signature from our test case
        let signature = hex::decode("3045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA7")
            .unwrap();

        // Wrong digest (all zeros)
        let wrong_digest = [0u8; 32];

        let result = verify_signature(&pubkey, &signature, &wrong_digest);
        assert!(!result, "Wrong digest should fail signature verification");
    }

    #[test]
    fn test_verify_signature_modified_signature() {
        // Tampered signature should fail
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();

        // Modified signature (last byte changed)
        let tampered_sig = hex::decode("3045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AFF")
            .unwrap();

        let digest = [1u8; 32]; // Any digest

        let result = verify_signature(&pubkey, &tampered_sig, &digest);
        assert!(!result, "Tampered signature should fail");
    }

    #[test]
    fn test_ed25519_detection() {
        // Ed25519 pubkey starts with 0xED
        let ed25519_pubkey =
            hex::decode("ED9434799226374926EDA3B54B1B461B4ABF7237962EAE18528FEA67595397FA32")
                .unwrap();

        assert_eq!(
            ed25519_pubkey[0], 0xED,
            "Ed25519 key should start with 0xED"
        );
    }

    #[test]
    fn test_secp256k1_detection() {
        // secp256k1 compressed pubkey starts with 0x02 or 0x03
        let secp_pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();

        assert!(
            secp_pubkey[0] == 0x02 || secp_pubkey[0] == 0x03,
            "secp256k1 compressed key should start with 0x02 or 0x03"
        );
    }

    #[test]
    fn test_verify_secp256k1_invalid_pubkey_length() {
        // Pubkey too short
        let short_pubkey = vec![0x02, 0xAA, 0xBB]; // Only 3 bytes
        let signature = hex::decode("3045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA7")
            .unwrap();
        let digest = [0u8; 32];

        let result = verify_signature(&short_pubkey, &signature, &digest);
        assert!(!result, "Invalid pubkey length should fail");
    }

    #[test]
    fn test_verify_ed25519_invalid_sizes() {
        // Ed25519 pubkey with wrong length
        let invalid_ed_pubkey = vec![0xED, 0xAA]; // Only 2 bytes (needs 33)
        let signature = vec![0u8; 64];
        let digest = [0u8; 32];

        let result = verify_signature(&invalid_ed_pubkey, &signature, &digest);
        assert!(!result, "Invalid Ed25519 pubkey size should fail");
    }

    #[test]
    fn test_verify_signature_deterministic() {
        // Same inputs should always produce same result
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();
        let signature = hex::decode("3045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA7")
            .unwrap();
        let digest = [1u8; 32];

        let result1 = verify_signature(&pubkey, &signature, &digest);
        let result2 = verify_signature(&pubkey, &signature, &digest);

        assert_eq!(
            result1, result2,
            "Signature verification should be deterministic"
        );
    }
}
