use crate::types::VerificationResult;

/// Prints the verification result in a nice format
pub fn print_verification_result(
    result: &VerificationResult,
    expected_address: &str,
    expected_challenge: Option<&str>,
) {
    println!();
    println!("🔐 XRPL SignIn Verification");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Address verification
    println!("📍 Address Verification:");
    println!("   Derived:  {}", result.derived_address);
    println!("   Expected: {}", expected_address);
    println!(
        "   Status:   {}",
        if result.address_valid {
            "✅ MATCH"
        } else {
            "❌ MISMATCH"
        }
    );
    println!();

    // Challenge verification
    if let Some(expected) = expected_challenge {
        if let Some(found) = &result.found_challenge {
            println!("🎫 Challenge Verification:");
            println!("   Found:    {}", found);
            println!("   Expected: {}", expected);
            println!(
                "   Status:   {}",
                if result.challenge_valid {
                    "✅ MATCH"
                } else {
                    "❌ MISMATCH"
                }
            );
        } else {
            println!("⚠️  No MemoData found in transaction");
        }
        println!();
    } else {
        println!("ℹ️  Challenge verification skipped (not provided)");
        println!();
    }

    // Signature verification
    println!("🔏 Cryptographic Signature:");
    println!(
        "   Status: {}",
        if result.signature_valid {
            "✅ VALID"
        } else {
            "❌ INVALID"
        }
    );
    println!();

    // Final result
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if result.is_valid() {
        println!("✅ AUTHENTICATION SUCCESSFUL");
        println!();
        println!("Security checks passed:");
        println!("  ✅ Address derivation matches (proves public key ownership)");
        if expected_challenge.is_some() {
            println!("  ✅ Challenge matches (prevents replay attacks)");
        }
        println!("  ✅ Cryptographic signature valid");
        println!();
        println!("This combination proves the user controls the private key");
        println!("for address {} and signed THIS specific challenge.", expected_address);
    } else {
        println!("❌ AUTHENTICATION FAILED");
        println!();
        if !result.address_valid {
            println!("  ❌ Address mismatch - wrong public key");
        }
        if !result.challenge_valid {
            println!("  ❌ Challenge mismatch - possible replay attack");
        }
        if !result.signature_valid {
            println!("  ❌ Cryptographic signature invalid");
        }
    }
}
