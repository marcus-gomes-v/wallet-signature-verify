use super::super::provider::{VerificationInput, WalletProvider};
use crate::types::VerificationResult;
use bs58::decode::Error as Bs58Error;
use ed25519_dalek::{Signature as EdSignature, Verifier, VerifyingKey};

/// Provider for Solana-style wallets (Ed25519 signatures)
pub struct SolanaProvider;

fn decode_base58(data: &str) -> Result<Vec<u8>, Bs58Error> {
    bs58::decode(data).into_vec()
}

fn validate_base58_pubkey(address: &str) -> anyhow::Result<Vec<u8>> {
    let pubkey = decode_base58(address).map_err(|e| anyhow::anyhow!("Solana: invalid address (base58 decode failed): {e}"))?;
    if pubkey.len() != 32 {
        return Err(anyhow::anyhow!(
            "Solana: address must decode to 32 bytes (got {})",
            pubkey.len()
        ));
    }
    Ok(pubkey)
}

fn validate_base58_signature(sig: &str) -> anyhow::Result<Vec<u8>> {
    let signature = decode_base58(sig).map_err(|e| anyhow::anyhow!("Solana: invalid signature (base58 decode failed): {e}"))?;
    if signature.len() != 64 {
        return Err(anyhow::anyhow!(
            "Solana: signature must decode to 64 bytes (got {})",
            signature.len()
        ));
    }
    Ok(signature)
}

impl WalletProvider for SolanaProvider {
    fn name(&self) -> &str {
        "Solana"
    }

    fn description(&self) -> &str {
        "Solana wallets (Ed25519 signatures)"
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        if input.challenge.is_none() {
            return Err(anyhow::anyhow!("Solana: challenge is required for verification"));
        }

        // Validate address and signature encodings/lengths
        validate_base58_pubkey(&input.expected_address)?;
        validate_base58_signature(&input.signature_data)?;

        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        let challenge = input
            .challenge
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Solana requires challenge"))?;

        // Decode address/pubkey
        let pubkey_bytes = validate_base58_pubkey(&input.expected_address)?;
        let derived_address = bs58::encode(&pubkey_bytes).into_string();
        let mut pubkey_array = [0u8; 32];
        pubkey_array.copy_from_slice(&pubkey_bytes);

        // Decode signature
        let signature_bytes = validate_base58_signature(&input.signature_data)?;
        let mut sig_arr: [u8; 64] = [0u8; 64];
        sig_arr.copy_from_slice(&signature_bytes);
        let signature = EdSignature::from_bytes(&sig_arr);

        // Build verifying key
        let vk = VerifyingKey::from_bytes(&pubkey_array).map_err(|e| {
            anyhow::anyhow!("Solana: failed to parse verifying key from address bytes: {e}")
        })?;

        // Verify signature over raw challenge
        let signature_valid = vk.verify(challenge.as_bytes(), &signature).is_ok();
        // If the signature verifies with this public key, it proves control of that address
        let address_valid = signature_valid;

        Ok(VerificationResult {
            address_valid,
            challenge_valid: true,
            signature_valid,
            derived_address,
            found_challenge: Some(challenge.clone()),
        })
    }
}
