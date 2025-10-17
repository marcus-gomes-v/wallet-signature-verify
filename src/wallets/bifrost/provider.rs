use super::super::provider::{VerificationInput, WalletProvider};
use super::core::verify_evm_signature;
use crate::types::VerificationResult;

/// Provider for Bifrost and other EVM-compatible wallets
pub struct BifrostProvider;

impl WalletProvider for BifrostProvider {
    fn name(&self) -> &str {
        "Bifrost Wallet"
    }

    fn description(&self) -> &str {
        "Bifrost - EVM-compatible wallet signature verification (Ethereum-style signatures)"
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        // Validate that we have a challenge
        if input.challenge.is_none() {
            return Err(anyhow::anyhow!(
                "Bifrost: challenge is required for verification"
            ));
        }

        // Validate that signature_data looks like an Ethereum signature
        // Should be 0x + 130 hex chars (65 bytes) or just 130 hex chars
        let sig = input.signature_data.trim_start_matches("0x");

        if sig.len() != 130 {
            return Err(anyhow::anyhow!(
                "Bifrost: signature_data must be 65 bytes (130 hex chars), got {} chars",
                sig.len()
            ));
        }

        // Validate that it's valid hex
        if !sig.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "Bifrost: signature_data must be valid hexadecimal"
            ));
        }

        // Validate that address looks like an Ethereum address
        let addr = input.expected_address.trim_start_matches("0x");
        if addr.len() != 40 {
            return Err(anyhow::anyhow!(
                "Bifrost: expected_address must be an Ethereum address (0x + 40 hex chars)"
            ));
        }

        if !addr.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "Bifrost: expected_address must be valid hexadecimal"
            ));
        }

        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        let challenge = input
            .challenge
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Bifrost requires challenge"))?;

        verify_evm_signature(&input.signature_data, challenge, &input.expected_address)
    }
}
