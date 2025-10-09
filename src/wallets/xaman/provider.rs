use super::super::provider::{VerificationInput, WalletProvider};
use crate::types::VerificationResult;
use crate::verify_xrpl_signin;

/// Provider for Xaman Wallet (XRPL SignIn)
pub struct XamanProvider;

impl WalletProvider for XamanProvider {
    fn name(&self) -> &str {
        "Xaman"
    }

    fn description(&self) -> &str {
        "Xaman Wallet (formerly Xumm) - XRPL SignIn transactions"
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        // Validate that signature_data looks like an XRPL hex blob
        if input.signature_data.len() < 100 {
            return Err(anyhow::anyhow!(
                "Xaman: signature_data too short (expected: complete XRPL hex blob)"
            ));
        }

        // Validate that it's valid hex
        if !input.signature_data.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "Xaman: signature_data must be valid hexadecimal"
            ));
        }

        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        verify_xrpl_signin(
            &input.signature_data,
            &input.expected_address,
            input.challenge.as_deref(),
        )
    }
}
