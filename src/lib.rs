//! # Wallet Signature Verify
//!
//! Universal wallet signature verifier using challenge-response authentication.
//!
//! This library validates that a specific wallet address actually signed a specific challenge,
//! providing a secure authentication mechanism for blockchain wallets.
//!
//! ## Features
//!
//! - `xaman` - Support for Xaman wallet (XRPL SignIn)
//! - `web3auth` - Support for Web3Auth wallet
//! - `cli` - CLI binary with env_logger
//! - `all-wallets` - Convenience feature to enable all wallets
//!
//! By default, all wallets and CLI are enabled. You can disable default features
//! and selectively enable only the wallets you need.
//!
//! ## Quick Start
//!
//! ```toml
//! # Cargo.toml - All wallets (default)
//! [dependencies]
//! wallet-signature-verify = "0.1"
//!
//! # Only Xaman wallet
//! [dependencies]
//! wallet-signature-verify = { version = "0.1", default-features = false, features = ["xaman"] }
//! ```
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use wallet_signature_verify::{
//!     wallets::{get_wallet_provider, WalletType, VerificationInput},
//! };
//!
//! fn main() -> anyhow::Result<()> {
//!     // Create verification input
//!     let input = VerificationInput {
//!         signature_data: "732102DB4811...".to_string(), // Signature hex
//!         expected_address: "rExampleAddr123456789xrpL1234567890".to_string(),
//!         challenge: Some("domain:timestamp:uuid:action:address".to_string()),
//!     };
//!
//!     // Get wallet provider
//!     let provider = get_wallet_provider(WalletType::Xaman);
//!
//!     // Verify signature
//!     let result = provider.verify(&input)?;
//!
//!     if result.is_valid() {
//!         println!("✅ Authentication successful!");
//!     } else {
//!         println!("❌ Authentication failed");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Challenge-Response Authentication
//!
//! This library implements a secure authentication pattern with 4 components:
//!
//! 1. **Challenge** - A unique message to be signed (e.g., "domain:timestamp:uuid:action:address")
//! 2. **Address** - The wallet address that signed the challenge
//! 3. **Signature** - The cryptographic signature (hex or DER format)
//! 4. **Wallet Type** - Which wallet was used (Xaman, Web3Auth, etc.)
//!
//! The verifier validates that the address actually signed the challenge, proving
//! ownership of the private key without exposing it.

pub mod crypto;
pub mod output;
pub mod parser;
pub mod types;
pub mod wallets;

use crate::crypto::{account_id_from_pubkey, sha512half, verify_signature};
use crate::parser::{extract_fields, reconstruct_unsigned_blob};
use crate::types::{TransactionFields, VerificationResult};

/// Verifies a complete XRPL SignIn signature
pub fn verify_xrpl_signin(
    signed_hex: &str,
    expected_address: &str,
    expected_challenge: Option<&str>,
) -> anyhow::Result<VerificationResult> {
    log::debug!("Starting XRPL SignIn verification");
    log::debug!("Expected address: {}", expected_address);
    log::debug!("Expected challenge: {:?}", expected_challenge);

    // Extract fields from signed blob
    let fields = extract_fields(signed_hex)?;
    log::debug!("Extracted fields from signed blob");

    if fields.signing_pubkey.is_empty() || fields.txn_signature.is_empty() {
        log::error!("Missing SigningPubKey or TxnSignature in blob");
        return Err(anyhow::anyhow!("Missing SigningPubKey or TxnSignature"));
    }

    log::debug!("SigningPubKey: {}", hex::encode(&fields.signing_pubkey));
    log::debug!("TxnSignature length: {} bytes", fields.txn_signature.len());

    // 1. Verify Address
    let account_id = account_id_from_pubkey(&fields.signing_pubkey);
    let derived_address = ripple_address_codec::encode_account_id(&account_id);
    let address_valid = derived_address == expected_address;

    log::info!("Address verification: {} (derived: {})",
        if address_valid { "VALID" } else { "INVALID" },
        derived_address
    );

    // 2. Verify Challenge (if provided)
    let (challenge_valid, found_challenge) = verify_challenge(&fields, expected_challenge);

    if expected_challenge.is_some() {
        log::info!("Challenge verification: {}",
            if challenge_valid { "VALID" } else { "INVALID" }
        );
    }

    // 3. Verify Signature
    let signature_valid = verify_cryptographic_signature(signed_hex, &fields)?;

    log::info!("Signature verification: {}",
        if signature_valid { "VALID" } else { "INVALID" }
    );

    Ok(VerificationResult {
        address_valid,
        challenge_valid,
        signature_valid,
        derived_address,
        found_challenge,
    })
}

/// Verifies the challenge in MemoData
fn verify_challenge(
    fields: &TransactionFields,
    expected_challenge: Option<&str>,
) -> (bool, Option<String>) {
    if let Some(expected) = expected_challenge {
        if !fields.memo_data.is_empty() {
            let memo_str = String::from_utf8_lossy(&fields.memo_data).to_string();
            let matches = memo_str == expected;
            (matches, Some(memo_str))
        } else {
            (false, None)
        }
    } else {
        // If no challenge provided, consider it valid (don't fail)
        (true, None)
    }
}

/// Verifies the cryptographic signature
fn verify_cryptographic_signature(
    signed_hex: &str,
    fields: &TransactionFields,
) -> anyhow::Result<bool> {
    let unsigned_prefixed = reconstruct_unsigned_blob(signed_hex)?;

    log::debug!("Unsigned blob reconstructed");
    log::debug!("Unsigned blob (hex): {}", hex::encode(&unsigned_prefixed));

    let digest = sha512half(&unsigned_prefixed);
    log::debug!("SHA-512Half digest calculated");

    let signature_valid = verify_signature(
        &fields.signing_pubkey,
        &fields.txn_signature,
        &digest,
    );

    Ok(signature_valid)
}
