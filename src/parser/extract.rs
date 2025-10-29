use crate::types::TransactionFields;
use hex::FromHex;

/// Extracts SigningPubKey, TxnSignature and MemoData from an XRPL hex blob
pub fn extract_fields(hex: &str) -> anyhow::Result<TransactionFields> {
    log::debug!("Extracting fields from XRPL hex blob");
    log::debug!("Hex blob length: {} chars", hex.len());

    let bytes = Vec::from_hex(hex)?;
    log::debug!("Decoded {} bytes from hex", bytes.len());

    let mut signing_pubkey = Vec::new();
    let mut txn_signature = Vec::new();
    let mut memo_data = Vec::new();
    let mut memo_type = Vec::new();

    // First pass: extract SigningPubKey and TxnSignature
    let mut i = 0;
    while i < bytes.len() {
        // Field 0x73 = SigningPubKey (only take the first one)
        if bytes[i] == 0x73 && i + 1 < bytes.len() && signing_pubkey.is_empty() {
            i += 1;
            let len = bytes[i] as usize;
            i += 1;
            if i + len <= bytes.len() {
                signing_pubkey = bytes[i..i + len].to_vec();
                log::debug!("Found SigningPubKey: {} bytes", signing_pubkey.len());
                i += len;
            }
            continue;
        }

        // Field 0x74 = TxnSignature (only take the first one)
        if bytes[i] == 0x74 && i + 1 < bytes.len() && txn_signature.is_empty() {
            i += 1;
            let len = bytes[i] as usize;
            i += 1;
            if i + len <= bytes.len() {
                txn_signature = bytes[i..i + len].to_vec();
                log::debug!("Found TxnSignature: {} bytes", txn_signature.len());
                i += len;
            }
            continue;
        }

        // Field 0xF9EA = Memo Array start - parse memo fields inside it
        if i + 1 < bytes.len() && bytes[i] == 0xF9 && bytes[i + 1] == 0xEA {
            log::debug!("Found Memo Array marker (0xF9EA) at position {}", i);
            i += 2; // Skip the 0xF9EA marker

            // Now parse memo fields within the array
            // The memo array continues until we hit 0xE1 or 0xF1 (end markers)
            while i < bytes.len() {
                // End of array markers
                if bytes[i] == 0xE1 || bytes[i] == 0xF1 {
                    log::debug!(
                        "Found end-of-array marker 0x{:02X} at position {}",
                        bytes[i],
                        i
                    );
                    break;
                }

                // Field 0x7C = MemoData
                if bytes[i] == 0x7C && i + 1 < bytes.len() {
                    i += 1;
                    let len = bytes[i] as usize;
                    i += 1;
                    if i + len <= bytes.len() {
                        memo_data = bytes[i..i + len].to_vec();
                        log::debug!(
                            "Found MemoData (0x7C) in memo array: {} bytes",
                            memo_data.len()
                        );
                        if let Ok(s) = String::from_utf8(memo_data.clone()) {
                            log::debug!("  Content: {}", s);
                        }
                        i += len;
                    }
                    continue;
                }

                // Field 0x7D = MemoType
                if bytes[i] == 0x7D && i + 1 < bytes.len() {
                    i += 1;
                    let len = bytes[i] as usize;
                    i += 1;
                    if i + len <= bytes.len() {
                        memo_type = bytes[i..i + len].to_vec();
                        log::debug!(
                            "Found MemoType (0x7D) in memo array: {} bytes",
                            memo_type.len()
                        );
                        if let Ok(s) = String::from_utf8(memo_type.clone()) {
                            log::debug!("  Content: {}", s);
                        }
                        i += len;
                    }
                    continue;
                }

                i += 1;
            }
            break; // Exit after processing memo array
        }

        i += 1;
    }

    // Choose which field contains the challenge:
    // - If MemoType (0x7D) is longer, it likely contains the challenge
    // - Otherwise use MemoData (0x7C)
    // This handles different wallet implementations
    let challenge_field = if memo_type.len() > memo_data.len() {
        log::debug!(
            "Using MemoType (0x7D) as challenge field ({} bytes)",
            memo_type.len()
        );
        memo_type
    } else {
        log::debug!(
            "Using MemoData (0x7C) as challenge field ({} bytes)",
            memo_data.len()
        );
        memo_data
    };

    log::debug!(
        "Field extraction complete: pubkey={} bytes, signature={} bytes, challenge_field={} bytes",
        signing_pubkey.len(),
        txn_signature.len(),
        challenge_field.len()
    );

    if !challenge_field.is_empty() {
        if let Ok(challenge_str) = String::from_utf8(challenge_field.clone()) {
            log::debug!("Challenge field (decoded): {}", challenge_str);
        }
    }

    Ok(TransactionFields {
        signing_pubkey,
        txn_signature,
        memo_data: challenge_field,
    })
}
