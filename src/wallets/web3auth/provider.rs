use super::super::provider::{VerificationInput, WalletProvider};
use super::core::verify_web3auth_signature;
use crate::types::VerificationResult;

/// Provider for Web3Auth (secp256k1 raw signatures)
pub struct Web3AuthProvider;

impl WalletProvider for Web3AuthProvider {
    fn name(&self) -> &str {
        "Web3Auth"
    }

    fn description(&self) -> &str {
        "Web3Auth - secp256k1 raw signature verification with public key recovery"
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        // Validate that we have a challenge
        if input.challenge.is_none() {
            return Err(anyhow::anyhow!(
                "Web3Auth: challenge is required for verification"
            ));
        }

        // Validate that signature_data looks like a DER signature
        if input.signature_data.len() < 64 {
            return Err(anyhow::anyhow!(
                "Web3Auth: signature_data too short (expected: DER hex signature)"
            ));
        }

        // Validate that it's valid hex
        if !input.signature_data.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "Web3Auth: signature_data must be valid hexadecimal"
            ));
        }

        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        let challenge = input
            .challenge
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Web3Auth requires challenge"))?;

        verify_web3auth_signature(&input.signature_data, challenge, &input.expected_address)
    }
}
