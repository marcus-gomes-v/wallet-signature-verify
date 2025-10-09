//! Integration tests for Xaman wallet signature verification
//!
//! These tests use REAL signatures from Xaman wallet to prove cryptographic security.
//! Philosophy: "Don't trust, verify" - Bitcoin style paranoid testing.

use wallet_signature_verify::verify_xrpl_signin;

/// Test with a REAL valid Xaman signature
#[test]
fn test_xaman_valid_signature_full_verification() {
    // This is a REAL signature from Xaman wallet
    let signed_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA781143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";

    let expected_address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";
    let expected_challenge = "nuff.tech:1760021404:d44b7372-8503-4b39-938b-8fc0c54d42b6:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

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
    let signed_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA781143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";

    let expected_address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";
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
    let signed_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA781143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";

    let wrong_address = "rATTACKERAddressXXXXXXXXXXXXXXXXXXXXXXX";
    let expected_challenge = "nuff.tech:1760021404:d44b7372-8503-4b39-938b-8fc0c54d42b6:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

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
    assert_eq!(result.derived_address, "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa");
}

/// Test with modified signature (cryptographic attack attempt)
#[test]
fn test_xaman_tampered_signature_rejected() {
    // Modified signature: changed last byte from A7 to FF
    let tampered_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AFF81143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";

    let expected_address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";
    let expected_challenge = "nuff.tech:1760021404:d44b7372-8503-4b39-938b-8fc0c54d42b6:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

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

    let expected_address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";
    let expected_challenge = "test:challenge";

    let result = verify_xrpl_signin(invalid_hex, expected_address, Some(expected_challenge));

    // Should return an error
    assert!(result.is_err(), "Invalid hex should return error");
}

/// Test with valid signature but no challenge provided
#[test]
fn test_xaman_no_challenge_validation() {
    let signed_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA781143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";

    let expected_address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

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

/// Test with a different REAL signature (second test vector)
#[test]
fn test_xaman_second_real_signature() {
    // Different real signature from the first test
    let signed_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574463044022001BFEFF7D1E37477962750D892E42BD971D3282A88BF863E40E29359715C641C022012778D96CBD8CB33C9CD3AC872B1A53509C8608443953837FF6B284807CD39BA81143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303031373736363A64326437356537372D366232312D343339362D616233382D6162666262656135313234303A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";

    let expected_address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";
    let expected_challenge = "nuff.tech:1760017766:d2d75e77-6b21-4396-ab38-abfbbea51240:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

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
}

/// Test that each signature is UNIQUE and cannot be replayed
#[test]
fn test_xaman_signatures_are_unique() {
    // First signature
    let sig1_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA781143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";
    let challenge1 = "nuff.tech:1760021404:d44b7372-8503-4b39-938b-8fc0c54d42b6:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

    // Second signature (different timestamp and UUID)
    let sig2_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574463044022001BFEFF7D1E37477962750D892E42BD971D3282A88BF863E40E29359715C641C022012778D96CBD8CB33C9CD3AC872B1A53509C8608443953837FF6B284807CD39BA81143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303031373736363A64326437356537372D366232312D343339362D616233382D6162666262656135313234303A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";
    let challenge2 = "nuff.tech:1760017766:d2d75e77-6b21-4396-ab38-abfbbea51240:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

    let address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

    // Sig1 should NOT work with Challenge2 (replay attack prevention)
    let replay_attempt = verify_xrpl_signin(sig1_hex, address, Some(challenge2))
        .expect("Verification should not error");

    assert!(
        !replay_attempt.challenge_valid,
        "Replay attack MUST be prevented"
    );
    assert!(!replay_attempt.is_valid(), "Replay attack MUST fail");

    // Sig2 should NOT work with Challenge1 (replay attack prevention)
    let replay_attempt2 = verify_xrpl_signin(sig2_hex, address, Some(challenge1))
        .expect("Verification should not error");

    assert!(
        !replay_attempt2.challenge_valid,
        "Replay attack MUST be prevented"
    );
    assert!(!replay_attempt2.is_valid(), "Replay attack MUST fail");
}

/// Stress test: Verify determinism (same inputs = same outputs)
#[test]
fn test_xaman_verification_is_deterministic() {
    let signed_hex = "732102DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A190574473045022100F9F3274CD7036053082EBDECEA98FCEF3125D1FCE881C903ED68D108760588FE022068F54A1AB529869E48B1A242903A23AC117A699AF65179BD545E532119E27AA781143680F8503E56B53239FE0F5EB782285B3FE4DDE8F9EA7C04417574687D626E7566662E746563683A313736303032313430343A64343462373337322D383530332D346233392D393338622D3866633063353464343262363A6C6F67696E3A726E79427A4D48626D4A4D7A7A686B344E6F797975714B7A73616866484669415261E1F1";
    let address = "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";
    let challenge = "nuff.tech:1760021404:d44b7372-8503-4b39-938b-8fc0c54d42b6:login:rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa";

    // Run verification 10 times
    for _ in 0..10 {
        let result = verify_xrpl_signin(signed_hex, address, Some(challenge))
            .expect("Verification should not error");

        assert!(result.is_valid(), "Verification must be deterministic");
        assert_eq!(result.derived_address, address);
    }
}
