//! Integration tests for Solana wallet signature verification
//!
//! Solana wallets use Ed25519 signatures over the raw challenge bytes.
//! These tests cover valid flows and common failure modes (wrong challenge, wrong address, invalid encodings, and determinism).

use bs58;
use ed25519_dalek::{Signer, SigningKey};
use wallet_signature_verify::wallets::{solana::SolanaProvider, VerificationInput, WalletProvider};

fn solana_fixture() -> (String, String, String) {
    // Deterministic test keypair
    let seed = [42u8; 32];
    let signing_key = SigningKey::from_bytes(&seed);
    let verify_key = signing_key.verifying_key();

    let challenge = "example.com:1760079290:uuid:login:solana_test_address";
    let signature = signing_key.sign(challenge.as_bytes());

    let address = bs58::encode(verify_key.to_bytes()).into_string();
    let signature_b58 = bs58::encode(signature.to_bytes()).into_string();

    (address, signature_b58, challenge.to_string())
}

#[test]
fn solana_valid_signature_passes() {
    let (address, signature_b58, challenge) = solana_fixture();
    let provider = SolanaProvider;

    let result = provider
        .verify(&VerificationInput {
            signature_data: signature_b58.clone(),
            expected_address: address.clone(),
            challenge: Some(challenge.clone()),
        })
        .expect("Solana verification should succeed for valid inputs");

    assert!(result.is_valid(), "All checks should pass");
    assert!(result.signature_valid, "Signature should be valid");
    assert!(result.address_valid, "Address should match recovered pubkey");
    assert_eq!(result.derived_address, address);
    assert_eq!(result.found_challenge.as_deref(), Some(challenge.as_str()));
}

#[test]
fn solana_wrong_challenge_fails() {
    let (address, signature_b58, _challenge) = solana_fixture();
    let provider = SolanaProvider;
    let wrong_challenge = "example.com:WRONG:challenge";

    let result = provider
        .verify(&VerificationInput {
            signature_data: signature_b58,
            expected_address: address,
            challenge: Some(wrong_challenge.to_string()),
        })
        .expect("Solana verification should return Ok even on mismatch");

    assert!(
        !result.signature_valid,
        "Signature must fail if challenge bytes differ"
    );
    assert!(!result.is_valid(), "Overall verification must fail");
}

#[test]
fn solana_wrong_address_fails() {
    let (_address, signature_b58, challenge) = solana_fixture();
    let provider = SolanaProvider;

    // Different deterministic keypair -> different address
    let alt_seed = [7u8; 32];
    let alt_signing_key = SigningKey::from_bytes(&alt_seed);
    let wrong_address = bs58::encode(alt_signing_key.verifying_key().to_bytes()).into_string();

    let result = provider
        .verify(&VerificationInput {
            signature_data: signature_b58,
            expected_address: wrong_address,
            challenge: Some(challenge),
        })
        .expect("Solana verification should return Ok even with wrong address");

    assert!(
        !result.signature_valid,
        "Signature should fail when verified against a different pubkey/address"
    );
    assert!(!result.address_valid, "Address validation should fail");
    assert!(!result.is_valid(), "Overall verification must fail");
}

#[test]
fn solana_invalid_signature_length_errors() {
    let (address, _signature_b58, challenge) = solana_fixture();
    let provider = SolanaProvider;

    let short_sig = "too_short_sig";

    let result = provider.verify(&VerificationInput {
        signature_data: short_sig.to_string(),
        expected_address: address,
        challenge: Some(challenge),
    });

    assert!(
        result.is_err(),
        "Invalid signature length should raise an error"
    );
}

#[test]
fn solana_invalid_base58_signature_errors() {
    let (address, _signature_b58, challenge) = solana_fixture();
    let provider = SolanaProvider;

    let invalid_sig = "!!!!not_base58!!!!";

    let result = provider.verify(&VerificationInput {
        signature_data: invalid_sig.to_string(),
        expected_address: address,
        challenge: Some(challenge),
    });

    assert!(
        result.is_err(),
        "Non-base58 signature should raise an error"
    );
}

#[test]
fn solana_missing_challenge_rejected() {
    let (address, signature_b58, _challenge) = solana_fixture();
    let provider = SolanaProvider;

    let result = provider.verify(&VerificationInput {
        signature_data: signature_b58,
        expected_address: address,
        challenge: None,
    });

    assert!(result.is_err(), "Challenge is required for Solana verification");
}

#[test]
fn solana_deterministic_verification() {
    let (address, signature_b58, challenge) = solana_fixture();
    let provider = SolanaProvider;

    let input = VerificationInput {
        signature_data: signature_b58,
        expected_address: address,
        challenge: Some(challenge),
    };

    let result1 = provider.verify(&input).expect("first verification should succeed");
    let result2 = provider.verify(&input).expect("second verification should succeed");

    assert_eq!(result1.is_valid(), result2.is_valid());
    assert_eq!(result1.signature_valid, result2.signature_valid);
    assert_eq!(result1.address_valid, result2.address_valid);
}
