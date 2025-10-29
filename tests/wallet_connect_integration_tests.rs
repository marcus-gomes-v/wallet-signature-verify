//! Integration tests for WalletConnect (EVM) signature verification
//!
//! WalletConnect and other EVM-compatible wallets use Ethereum-style signatures (secp256k1)
//! with EIP-191 message signing format.
//!
//! These tests verify that signatures are cryptographically validated using real test data.

use wallet_signature_verify::wallets::wallet_connect::core::verify_evm_signature;

/// Test WalletConnect with a real, valid signature
///
/// This test uses actual signature data from a WalletConnect wallet that was validated
/// to work correctly with the wallet-signature-verify binary.
#[test]
fn test_wallet_connect_valid_signature() {
    // Real test data from successful verification
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(signature, challenge, address);

    assert!(
        result.is_ok(),
        "Valid WalletConnect signature should verify successfully"
    );

    let verification = result.unwrap();
    assert!(
        verification.is_valid(),
        "All verification checks should pass"
    );
    assert!(verification.address_valid, "Address should be valid");
    assert!(verification.challenge_valid, "Challenge should be valid");
    assert!(
        verification.signature_valid,
        "Signature should be cryptographically valid"
    );
}

/// Test WalletConnect signature verification without 0x prefix
#[test]
fn test_wallet_connect_signature_without_0x_prefix() {
    let signature = "e5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(signature, challenge, address);

    // Should work without 0x prefix as well
    assert!(
        result.is_ok(),
        "Signature without 0x prefix should also work"
    );
    assert!(result.unwrap().is_valid(), "Verification should pass");
}

/// Test WalletConnect with wrong address (signature is valid but for different address)
#[test]
fn test_wallet_connect_wrong_address() {
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
    let wrong_address = "0x0000000000000000000000000000000000000000";

    let result = verify_evm_signature(signature, challenge, wrong_address);

    assert!(
        result.is_ok(),
        "Should not error, but validation should fail"
    );

    let verification = result.unwrap();
    assert!(
        !verification.is_valid(),
        "Verification should fail with wrong address"
    );
    assert!(!verification.address_valid, "Address check should fail");
}

/// Test WalletConnect with wrong challenge (signature is for different challenge)
#[test]
fn test_wallet_connect_wrong_challenge() {
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let wrong_challenge =
        "nuff.tech:9999999999:different:challenge:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(signature, wrong_challenge, address);

    assert!(
        result.is_ok(),
        "Should not error, but validation should fail"
    );

    let verification = result.unwrap();
    assert!(
        !verification.is_valid(),
        "Verification should fail with wrong challenge"
    );
}

/// Test WalletConnect with invalid signature format (too short)
#[test]
fn test_wallet_connect_invalid_signature_length() {
    let invalid_signature = "0xe5092134a1e3a91dafe709"; // Too short
    let challenge = "test:challenge";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(invalid_signature, challenge, address);

    assert!(
        result.is_err(),
        "Invalid signature length should return error"
    );
}

/// Test WalletConnect with malformed signature hex
#[test]
fn test_wallet_connect_invalid_hex() {
    let invalid_hex = "0xZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ"; // Invalid hex characters
    let challenge = "test:challenge";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(invalid_hex, challenge, address);

    assert!(result.is_err(), "Invalid hex should return error");
}

/// Test WalletConnect address case insensitivity
/// Ethereum addresses are case-insensitive for comparison
#[test]
fn test_wallet_connect_address_case_insensitive() {
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    // Test with lowercase address
    let address_lowercase = "0x33f9d9f0348c1a4bace2ad839903bbd47f430651";
    let result_lower = verify_evm_signature(signature, challenge, address_lowercase);
    assert!(
        result_lower.is_ok() && result_lower.unwrap().is_valid(),
        "Lowercase address should work"
    );

    // Test with uppercase address
    let address_uppercase = "0x33F9D9F0348C1A4BACE2AD839903BBD47F430651";
    let result_upper = verify_evm_signature(signature, challenge, address_uppercase);
    assert!(
        result_upper.is_ok() && result_upper.unwrap().is_valid(),
        "Uppercase address should work"
    );
}

/// Test that WalletConnect verification is deterministic
/// Same inputs should always produce same outputs
#[test]
fn test_wallet_connect_deterministic() {
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    // Run verification multiple times
    let result1 = verify_evm_signature(signature, challenge, address);
    let result2 = verify_evm_signature(signature, challenge, address);
    let result3 = verify_evm_signature(signature, challenge, address);

    // All should succeed
    assert!(result1.is_ok() && result2.is_ok() && result3.is_ok());

    // All should have same validity
    let valid1 = result1.unwrap().is_valid();
    let valid2 = result2.unwrap().is_valid();
    let valid3 = result3.unwrap().is_valid();

    assert_eq!(valid1, valid2);
    assert_eq!(valid2, valid3);
}

/// Test WalletConnect with empty challenge
#[test]
fn test_wallet_connect_empty_challenge() {
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let empty_challenge = "";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(signature, empty_challenge, address);

    // Should not panic, but validation should fail
    assert!(result.is_ok(), "Should not panic with empty challenge");
    assert!(
        !result.unwrap().is_valid(),
        "Should fail validation with empty challenge"
    );
}

/// Test WalletConnect signature format (65 bytes: r + s + v)
#[test]
fn test_wallet_connect_signature_components() {
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";

    // Remove 0x prefix
    let sig_hex = signature.strip_prefix("0x").unwrap_or(signature);

    // Should be exactly 130 hex characters (65 bytes)
    assert_eq!(
        sig_hex.len(),
        130,
        "Signature should be 130 hex chars (65 bytes)"
    );

    // Should be valid hex
    assert!(
        sig_hex.chars().all(|c| c.is_ascii_hexdigit()),
        "Should be valid hex"
    );
}

/// Test that WalletConnect uses EIP-191 message format
/// This is implicitly tested by the fact that our known signature works
#[test]
fn test_wallet_connect_eip191_format() {
    // The fact that our real signature validates proves EIP-191 format is being used
    // because that's the standard for personal_sign in Ethereum wallets
    let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
    let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
    let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

    let result = verify_evm_signature(signature, challenge, address);

    // If this passes, it confirms EIP-191 is being used
    assert!(
        result.is_ok() && result.unwrap().is_valid(),
        "Valid EIP-191 signature should verify"
    );
}
