//! WalletConnect Provider
//!
//! This module provides signature verification for WalletConnect and other EVM-compatible wallets.
//! It supports Ethereum-style signatures (secp256k1) with EIP-191 message signing.
//!
//! # Supported Wallets
//!
//! - WalletConnect
//! - MetaMask
//! - Bifrost Wallet
//! - Any EVM-compatible wallet using `personal_sign`
//!
//! # Signature Format
//!
//! Uses EIP-191 (Ethereum Signed Message):
//! - Message is prefixed with: `\x19Ethereum Signed Message:\n{length}{message}`
//! - Signature is 65 bytes: r (32 bytes) + s (32 bytes) + v (1 byte)
//! - Recovery ID (v) is used to recover the public key
//!
//! # Example
//!
//! ```rust,no_run
//! use wallet_signature_verify::wallets::{get_wallet_provider, WalletType, VerificationInput};
//!
//! let input = VerificationInput {
//!     signature_data: "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b".to_string(),
//!     expected_address: "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651".to_string(),
//!     challenge: Some("nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651".to_string()),
//! };
//!
//! let provider = get_wallet_provider(WalletType::WalletConnect);
//! let result = provider.verify(&input)?;
//!
//! assert!(result.is_valid());
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod core;
mod provider;

pub use provider::WalletConnectProvider;
