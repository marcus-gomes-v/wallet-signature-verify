use crate::types::VerificationResult;

/// Input for signature verification (flexible for different wallets)
#[derive(Debug, Clone)]
pub struct VerificationInput {
    /// Signature or hex blob (format varies by wallet)
    pub signature_data: String,
    /// Expected address
    pub expected_address: String,
    /// Challenge that was signed
    pub challenge: Option<String>,
}

/// Trait that each wallet provider must implement
pub trait WalletProvider: Send + Sync {
    /// Wallet name (e.g., "Xaman", "Web3Auth")
    fn name(&self) -> &str;

    /// Wallet description
    fn description(&self) -> &str;

    /// Verifies the signature
    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult>;

    /// Validates if the input is valid for this provider
    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()>;
}
