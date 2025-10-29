use super::super::provider::{VerificationInput, WalletProvider};
use super::core::verify_evm_signature;
use crate::types::VerificationResult;

/// Provider for WalletConnect and other EVM-compatible wallets
///
/// # Overview
///
/// WalletConnectProvider implements signature verification for Ethereum-style wallets
/// including WalletConnect, MetaMask, Bifrost, and other EVM-compatible wallets.
///
/// # Signature Format
///
/// Uses EIP-191 personal_sign format:
/// - Signature: 65 bytes (r + s + v) = 130 hex characters
/// - Address: Ethereum address (0x + 40 hex characters)
/// - Message: Any UTF-8 string (hashed with EIP-191 prefix)
///
/// # Example
///
/// ```rust,no_run
/// use wallet_signature_verify::wallets::{get_wallet_provider, WalletType, VerificationInput};
///
/// let input = VerificationInput {
///     signature_data: "0xe5092134a1e3a91dafe7095916466a00...".to_string(),
///     expected_address: "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651".to_string(),
///     challenge: Some("nuff.tech:1760706960:uuid:login:0x...".to_string()),
/// };
///
/// let provider = get_wallet_provider(WalletType::WalletConnect);
/// let result = provider.verify(&input)?;
///
/// if result.is_valid() {
///     println!("✅ Valid Ethereum signature");
/// }
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct WalletConnectProvider;

impl WalletProvider for WalletConnectProvider {
    fn name(&self) -> &str {
        "WalletConnect"
    }

    fn description(&self) -> &str {
        "WalletConnect - EVM-compatible wallet signature verification (Ethereum-style signatures)"
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        // Validate that we have a challenge
        if input.challenge.is_none() {
            return Err(anyhow::anyhow!(
                "WalletConnect: challenge is required for verification"
            ));
        }

        // Validate that signature_data looks like an Ethereum signature
        // Should be 0x + 130 hex chars (65 bytes) or just 130 hex chars
        let sig = input.signature_data.trim_start_matches("0x");

        if sig.len() != 130 {
            return Err(anyhow::anyhow!(
                "WalletConnect: signature_data must be 65 bytes (130 hex chars), got {} chars",
                sig.len()
            ));
        }

        // Validate that it's valid hex
        if !sig.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "WalletConnect: signature_data must be valid hexadecimal"
            ));
        }

        // Validate that address looks like an Ethereum address
        let addr = input.expected_address.trim_start_matches("0x");
        if addr.len() != 40 {
            return Err(anyhow::anyhow!(
                "WalletConnect: expected_address must be an Ethereum address (0x + 40 hex chars)"
            ));
        }

        if !addr.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(anyhow::anyhow!(
                "WalletConnect: expected_address must be valid hexadecimal"
            ));
        }

        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        let challenge = input
            .challenge
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("WalletConnect requires challenge"))?;

        verify_evm_signature(&input.signature_data, challenge, &input.expected_address)
    }
}
