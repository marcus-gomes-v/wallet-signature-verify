//! Simple usage example of wallet-signature-verify library
//!
//! Run with: cargo run --example simple_usage

use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    println!("🔐 Wallet Signature Verify - Simple Usage Example\n");

    // Example 1: Xaman wallet verification
    #[cfg(feature = "xaman")]
    {
        println!("📱 Example 1: Xaman Wallet Verification");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        let input = VerificationInput {
            signature_data: "7321ED9434799FED...TRUNCATED...E1F1".to_string(),
            expected_address: "rExampleAddr123456789xrpL1234567890".to_string(),
            challenge: Some("example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890".to_string()),
        };

        let provider = get_wallet_provider(WalletType::Xaman);
        println!("Using provider: {}", provider.name());
        println!("Description: {}\n", provider.description());

        match provider.verify(&input) {
            Ok(result) => {
                println!("Verification Result:");
                println!("  Address valid:   {}", if result.address_valid { "✅" } else { "❌" });
                println!("  Challenge valid: {}", if result.challenge_valid { "✅" } else { "❌" });
                println!("  Signature valid: {}", if result.signature_valid { "✅" } else { "❌" });
                println!("  Derived address: {}", result.derived_address);

                if result.is_valid() {
                    println!("\n✅ Authentication successful!");
                } else {
                    println!("\n❌ Authentication failed");
                }
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }

        println!("\n");
    }

    // Example 2: Dynamic wallet type parsing
    println!("🔄 Example 2: Dynamic Wallet Type Selection");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let wallet_str = "xaman"; // Could come from user input
    match WalletType::from_str(wallet_str) {
        Ok(wallet_type) => {
            println!("✅ Parsed wallet type: {:?}", wallet_type);
            let provider = get_wallet_provider(wallet_type);
            println!("   Provider: {}", provider.name());
        }
        Err(e) => {
            println!("❌ {}", e);
        }
    }

    println!("\n");

    // Example 3: List supported wallets
    println!("📋 Example 3: Supported Wallets");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let supported = WalletType::supported_wallets();
    println!("Enabled wallets in this build:");
    for wallet in &supported {
        println!("  - {}", wallet);
    }

    if supported.is_empty() {
        println!("  ⚠️  No wallets enabled! Enable at least one wallet feature.");
    }

    println!("\n");

    // Example 4: Web3Auth (if enabled)
    #[cfg(feature = "web3auth")]
    {
        println!("🌐 Example 4: Web3Auth Wallet (feature enabled)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        println!("Web3Auth provider is available!");
        println!("You can verify Web3Auth signatures with DER format.\n");
    }

    #[cfg(not(feature = "web3auth"))]
    {
        println!("🌐 Example 4: Web3Auth Wallet (feature disabled)");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        println!("Web3Auth is not enabled in this build.");
        println!("Enable with: --features web3auth\n");
    }

    Ok(())
}
