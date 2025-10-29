use crate::types::VerificationResult;
use ethers_core::{types::Signature, utils::hash_message};
use hex::FromHex;

/// Verifies an EVM (Ethereum-style) signature
///
/// This function verifies signatures from EVM-compatible wallets like WalletConnect, MetaMask, and Bifrost.
/// It uses EIP-191 personal_sign message format.
pub fn verify_evm_signature(
    signature_hex: &str,
    challenge: &str,
    expected_address: &str,
) -> anyhow::Result<VerificationResult> {
    log::debug!("EVM verification starting");
    log::debug!("Signature: {}", signature_hex);
    log::debug!("Challenge: {}", challenge);
    log::debug!("Expected Address: {}", expected_address);

    // Remove 0x prefix if present
    let signature_hex = signature_hex.trim_start_matches("0x");
    let expected_address = expected_address.trim_start_matches("0x");

    // Parse signature (65 bytes: r(32) + s(32) + v(1))
    let signature_bytes = Vec::from_hex(signature_hex)
        .map_err(|e| anyhow::anyhow!("Failed to decode signature hex: {}", e))?;

    if signature_bytes.len() != 65 {
        return Err(anyhow::anyhow!(
            "Invalid signature length: expected 65 bytes, got {}",
            signature_bytes.len()
        ));
    }

    // Parse the signature using ethers
    let signature: Signature = signature_bytes
        .as_slice()
        .try_into()
        .map_err(|e| anyhow::anyhow!("Failed to parse signature: {:?}", e))?;

    log::debug!("Signature parsed successfully");
    log::debug!("  v: {}", signature.v);

    // Hash the message using EIP-191 format (what personal_sign does)
    // This adds the prefix: "\x19Ethereum Signed Message:\n" + len(message) + message
    let message_hash = hash_message(challenge.as_bytes());
    log::debug!("Message hash: 0x{}", hex::encode(message_hash.as_bytes()));

    // Recover the address from the signature
    let recovered_address = signature
        .recover(message_hash)
        .map_err(|e| anyhow::anyhow!("Failed to recover address from signature: {}", e))?;

    let recovered_address_str = format!("{:?}", recovered_address).to_lowercase();
    let expected_address_str = format!("0x{}", expected_address).to_lowercase();

    log::debug!("Recovered address: {}", recovered_address_str);
    log::debug!("Expected address:  {}", expected_address_str);

    let address_valid = recovered_address_str == expected_address_str;

    if address_valid {
        log::info!("Signature verification successful!");
        log::info!("Address matches: {}", recovered_address_str);
    } else {
        log::warn!("Signature verification failed!");
        log::warn!("Recovered address does not match expected address");
    }

    Ok(VerificationResult {
        address_valid,
        challenge_valid: true,          // Challenge is what we signed
        signature_valid: address_valid, // If address matches, signature is valid
        derived_address: recovered_address_str,
        found_challenge: Some(challenge.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_signature_verification() {
        // This is a test signature - in real usage, you'd get this from a wallet
        let signature = "0xe5092134a1e3a91dafe7095916466a00d93fa01c540914fc3a010c05220281eb1f8fbcb34ce784875cd4a01cabef782c3c0f7e33d508410e957fb01c1c5b10071b";
        let challenge = "nuff.tech:1760706960:afba42ef-fbb7-4504-8915-583046d6eb26:login:0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";
        let address = "0x33f9D9f0348c1a4Bace2ad839903bBD47F430651";

        let result = verify_evm_signature(signature, challenge, address);

        // This test should pass if the signature is valid for the given challenge and address
        match result {
            Ok(r) => {
                println!("Verification result: {:?}", r);
                assert!(r.is_valid(), "Expected signature to be valid");
            }
            Err(e) => {
                panic!("Verification failed: {}", e);
            }
        }
    }
}
