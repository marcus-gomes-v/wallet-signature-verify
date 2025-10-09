use ripemd::Ripemd160;
use sha2::{Digest, Sha256, Sha512};

/// Calculates SHA-512 and returns the first half (32 bytes)
pub fn sha512half(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut half = [0u8; 32];
    half.copy_from_slice(&out[..32]);
    half
}

/// Derives the Account ID (20 bytes) from a public key
/// using SHA-256 followed by RIPEMD-160
pub fn account_id_from_pubkey(pubkey: &[u8]) -> [u8; 20] {
    let sha = Sha256::digest(pubkey);
    let ripemd = Ripemd160::digest(sha);
    let mut out = [0u8; 20];
    out.copy_from_slice(&ripemd[..20]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha512half_empty_input() {
        // Test with empty input
        let input = b"";
        let result = sha512half(input);

        // SHA-512Half of empty string (first 32 bytes of SHA-512(""))
        let expected =
            hex::decode("cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce")
                .unwrap();

        assert_eq!(result.to_vec(), expected);
    }

    #[test]
    fn test_sha512half_known_value() {
        // Test with "hello world"
        let input = b"hello world";
        let result = sha512half(input);

        // First 32 bytes of SHA-512("hello world")
        let expected =
            hex::decode("309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f")
                .unwrap();

        assert_eq!(result.to_vec(), expected);
    }

    #[test]
    fn test_sha512half_xrpl_signing_prefix() {
        // Test with XRPL signing prefix "STX\0"
        let input = b"STX\0test";
        let result = sha512half(input);

        // Should return 32 bytes
        assert_eq!(result.len(), 32);
    }

    #[test]
    fn test_account_id_from_pubkey_secp256k1() {
        // Real XRPL public key (secp256k1) from our test case
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();

        let account_id = account_id_from_pubkey(&pubkey);

        // This should derive to rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa
        let address = ripple_address_codec::encode_account_id(&account_id);
        assert_eq!(address, "rnyBzMHbmJMzzhk4NoyyuqKzsahfHFiARa");
    }

    #[test]
    fn test_account_id_from_pubkey_ed25519() {
        // Test with Ed25519 public key (starts with 0xED)
        // This is a known test vector from XRPL
        let pubkey =
            hex::decode("ED9434799226374926EDA3B54B1B461B4ABF7237962EAE18528FEA67595397FA32")
                .unwrap();

        let account_id = account_id_from_pubkey(&pubkey);

        // Should derive to rDTXLQ7ZKZVKz33zJbHjgVShjsBnqMBhmN
        let address = ripple_address_codec::encode_account_id(&account_id);
        assert_eq!(address, "rDTXLQ7ZKZVKz33zJbHjgVShjsBnqMBhmN");
    }

    #[test]
    fn test_account_id_length() {
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();

        let account_id = account_id_from_pubkey(&pubkey);

        // Account ID must always be 20 bytes
        assert_eq!(account_id.len(), 20);
    }

    #[test]
    fn test_sha512half_deterministic() {
        // Same input should always produce same output
        let input = b"test message";
        let result1 = sha512half(input);
        let result2 = sha512half(input);

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_account_id_deterministic() {
        // Same public key should always produce same account ID
        let pubkey =
            hex::decode("02DB48115142459C05AA0D26F3752ADC9C5AF8348ADCF22A8CA73D5DF1839A1905")
                .unwrap();

        let account_id1 = account_id_from_pubkey(&pubkey);
        let account_id2 = account_id_from_pubkey(&pubkey);

        assert_eq!(account_id1, account_id2);
    }
}
