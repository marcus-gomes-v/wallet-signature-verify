// Example usage of the wallet-signature-verify library in Rust
//
// This file shows how to integrate the signature verifier
// directly into another Rust project.
//
// To use in another project, add to Cargo.toml:
// ```toml
// [dependencies]
// wallet-signature-verify = { path = "../wallet-signature-verify" }
// ```

use wallet_signature_verify::{
    wallets::{get_wallet_provider, registry::WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    println!("🔐 Wallet Signature Verifier - Rust Library Example\n");

    // ============================================
    // Example 1: Xaman Wallet
    // ============================================
    println!("📝 Example 1: Verifying Xaman Wallet...");

    let xaman_input = VerificationInput {
        signature_data: "7321ED9434799FED...TRUNCATED...E1F1".to_string(),
        expected_address: "rExampleAddr123456789xrpL1234567890".to_string(),
        challenge: Some("example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890".to_string()),
    };

    let xaman_provider = get_wallet_provider(WalletType::Xaman);

    match xaman_provider.verify(&xaman_input) {
        Ok(result) if result.is_valid() => {
            println!("   ✅ Xaman: Authentication VALID");
            println!("   → Derived address: {}", result.derived_address);
            println!("   → Challenge valid: {}\n", result.challenge_valid);
        }
        Ok(_) => {
            println!("   ❌ Xaman: Authentication FAILED\n");
        }
        Err(e) => {
            println!("   ⚠️  Xaman: Error - {}\n", e);
        }
    }

    // ============================================
    // Example 2: Web3Auth
    // ============================================
    println!("📝 Example 2: Verifying Web3Auth...");

    let web3auth_input = VerificationInput {
        signature_data: "3045022100ABC123...TRUNCATED...7890".to_string(),
        expected_address: "rTestAddr789012345xrpLTest567890abc".to_string(),
        challenge: Some("example.com:1234567891:87654321-dcba-4321-dcba-987654321cba:login:rTestAddr789012345xrpLTest567890abc".to_string()),
    };

    let web3auth_provider = get_wallet_provider(WalletType::Web3Auth);

    match web3auth_provider.verify(&web3auth_input) {
        Ok(result) if result.is_valid() => {
            println!("   ✅ Web3Auth: Authentication VALID");
            println!("   → Derived address: {}", result.derived_address);
            println!("   → Challenge valid: {}\n", result.challenge_valid);
        }
        Ok(_) => {
            println!("   ❌ Web3Auth: Authentication FAILED\n");
        }
        Err(e) => {
            println!("   ⚠️  Web3Auth: Error - {}\n", e);
        }
    }

    // ============================================
    // Example 3: Parse WalletType from String
    // ============================================
    println!("📝 Example 3: Dynamic wallet type parsing...");

    let wallet_type_str = "xaman"; // Could come from user input

    match WalletType::from_str(wallet_type_str) {
        Ok(wallet_type) => {
            let provider = get_wallet_provider(wallet_type);
            println!("   ✅ Wallet '{}' supported: {}", wallet_type_str, provider.description());
        }
        Err(e) => {
            println!("   ❌ Error: {}", e);
            println!("   💡 Supported wallets: {:?}", WalletType::supported_wallets());
        }
    }

    println!("\n🎉 Examples completed!");
    Ok(())
}

// ============================================
// Example of reusable function
// ============================================

/// Verifies a signature in a simplified way
///
/// # Example
/// ```rust
/// if verify_user_login("xaman", signature, address, Some(challenge))? {
///     // Authenticate user
/// }
/// ```
#[allow(dead_code)]
fn verify_user_login(
    wallet_type: &str,
    signature: String,
    address: String,
    challenge: Option<String>,
) -> anyhow::Result<bool> {
    let wallet_type = WalletType::from_str(wallet_type)
        .map_err(|e| anyhow::anyhow!(e))?;

    let provider = get_wallet_provider(wallet_type);

    let input = VerificationInput {
        signature_data: signature,
        expected_address: address,
        challenge,
    };

    let result = provider.verify(&input)?;
    Ok(result.is_valid())
}
