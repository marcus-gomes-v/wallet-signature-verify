//! Integration tests for Xaman wallet signature verification
//!
//! These tests use REAL signatures from Xaman wallet to prove cryptographic security.
//! Philosophy: "Don't trust, verify" - Bitcoin style paranoid testing.
//!
//! NOTE: All test data uses example.com domain and test addresses - NO production data.

use wallet_signature_verify::verify_xrpl_signin;

/// Test with a REAL valid Xaman signature
#[test]
fn test_xaman_valid_signature_full_verification() {
    // This is a REAL signature from Xaman wallet (using test/example data)
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let expected_challenge = "example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    let result = verify_xrpl_signin(signed_hex, expected_address, Some(expected_challenge))
        .expect("Verification should not error");

    // ALL checks must pass
    assert!(result.address_valid, "Address verification must pass");
    assert!(result.challenge_valid, "Challenge verification must pass");
    assert!(
        result.signature_valid,
        "Cryptographic signature must be valid"
    );
    assert!(result.is_valid(), "Overall verification must pass");

    // Verify derived address matches
    assert_eq!(result.derived_address, expected_address);

    // Verify challenge was correctly extracted
    assert_eq!(result.found_challenge, Some(expected_challenge.to_string()));
}

/// Test that wrong challenge is REJECTED (prevents replay attacks)
#[test]
fn test_xaman_wrong_challenge_rejected() {
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let wrong_challenge = "ATTACKER:trying:to:replay:attack";

    let result = verify_xrpl_signin(signed_hex, expected_address, Some(wrong_challenge))
        .expect("Verification should not error");

    // Signature is valid but challenge doesn't match - MUST FAIL
    assert!(result.address_valid, "Address is still valid");
    assert!(
        result.signature_valid,
        "Signature is cryptographically valid"
    );
    assert!(!result.challenge_valid, "Challenge MUST NOT match");
    assert!(!result.is_valid(), "Overall verification MUST FAIL");
}

/// Test that wrong address is REJECTED
#[test]
fn test_xaman_wrong_address_rejected() {
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let wrong_address = "rATTACKERAddressXXXXXXXXXXXXXXXXXXXXXXX";
    let expected_challenge = "example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    let result = verify_xrpl_signin(signed_hex, wrong_address, Some(expected_challenge))
        .expect("Verification should not error");

    // Signature and challenge are valid but address doesn't match - MUST FAIL
    assert!(!result.address_valid, "Address MUST NOT match");
    assert!(result.challenge_valid, "Challenge is valid");
    assert!(
        result.signature_valid,
        "Signature is cryptographically valid"
    );
    assert!(!result.is_valid(), "Overall verification MUST FAIL");

    // The derived address should be the REAL one from the signature
    assert_eq!(result.derived_address, "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU");
}

/// Test with modified signature (cryptographic attack attempt)
#[test]
fn test_xaman_tampered_signature_rejected() {
    // Modified signature: changed last byte from D2 to FF
    let tampered_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17FF8114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let expected_challenge = "example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    let result = verify_xrpl_signin(tampered_hex, expected_address, Some(expected_challenge))
        .expect("Verification should not error");

    // Cryptographic verification MUST catch the tampering
    assert!(
        !result.signature_valid,
        "Tampered signature MUST be detected as invalid"
    );
    assert!(!result.is_valid(), "Overall verification MUST FAIL");
}

/// Test with completely invalid hex (malformed data)
#[test]
fn test_xaman_invalid_hex_data() {
    let invalid_hex = "ZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZZ"; // Invalid hex chars

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let expected_challenge = "test:challenge";

    let result = verify_xrpl_signin(invalid_hex, expected_address, Some(expected_challenge));

    // Should return an error
    assert!(result.is_err(), "Invalid hex should return error");
}

/// Test with valid signature but no challenge provided
#[test]
fn test_xaman_no_challenge_validation() {
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    // No challenge provided
    let result = verify_xrpl_signin(signed_hex, expected_address, None)
        .expect("Verification should not error");

    // Should pass when no challenge is required
    assert!(result.address_valid, "Address verification must pass");
    assert!(
        result.challenge_valid,
        "Challenge should be considered valid when not required"
    );
    assert!(
        result.signature_valid,
        "Cryptographic signature must be valid"
    );
    assert!(result.is_valid(), "Overall verification must pass");
}

/// Test with a different challenge format (tests the parser edge case)
#[test]
fn test_xaman_address_with_0x73_byte() {
    // This signature is special: the address rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU
    // contains byte 0x73 in its Account ID, which was triggering a parser bug.
    // This test ensures the fix works correctly.
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let expected_challenge = "example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    let result = verify_xrpl_signin(signed_hex, expected_address, Some(expected_challenge))
        .expect("Verification should not error");

    // ALL checks must pass (this was failing before the parser fix)
    assert!(result.address_valid, "Address verification must pass");
    assert!(result.challenge_valid, "Challenge verification must pass");
    assert!(
        result.signature_valid,
        "Cryptographic signature must be valid"
    );
    assert!(result.is_valid(), "Overall verification must pass");
}

/// Test that signatures with different challenges cannot be replayed
#[test]
fn test_xaman_different_challenges_unique() {
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";

    let expected_address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let correct_challenge = "example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let different_challenge = "example.com:9999999999:different-uuid-here:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    // Correct challenge should work
    let result_correct = verify_xrpl_signin(signed_hex, expected_address, Some(correct_challenge))
        .expect("Verification should not error");
    assert!(result_correct.is_valid(), "Correct challenge must pass");

    // Different challenge should fail (replay attack prevention)
    let result_different =
        verify_xrpl_signin(signed_hex, expected_address, Some(different_challenge))
            .expect("Verification should not error");
    assert!(
        !result_different.challenge_valid,
        "Different challenge must fail"
    );
    assert!(!result_different.is_valid(), "Replay attack must be prevented");
}

/// Stress test: Verify determinism (same inputs = same outputs)
#[test]
fn test_xaman_verification_is_deterministic() {
    let signed_hex = "732102ACE0AE76CC7DA925442A417FA3618811B5043A66566C9909503D22A96514B2B87446304402207E22A82A87D5FCBFBD63BA78E078DFF6708923F90F96696C978B7C00FF4C71870220547C1C18010E8EA93E0D9D187DF4D93480C43CF19D54A49E69E12ED49CCE17D28114717251C1BFE144D8E3577777E6D04E1101E87336F9EA7C04417574687D636578616D706C652E636F6D3A313736303037393239303A35376530363130322D633063382D346366382D626539372D3533306332353135613535643A73796E6B3A72424C694A6A6E4768517238743144555358576676634E577858356D506956535755E1F1";
    let address = "rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";
    let challenge = "example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:synk:rBLiJjnGhQr8t1DUSXWfvcNWxX5mPiVSWU";

    // Run verification 10 times
    for _ in 0..10 {
        let result = verify_xrpl_signin(signed_hex, address, Some(challenge))
            .expect("Verification should not error");

        assert!(result.is_valid(), "Verification must be deterministic");
        assert_eq!(result.derived_address, address);
    }
}
