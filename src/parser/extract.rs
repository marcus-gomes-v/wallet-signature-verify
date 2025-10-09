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

    let mut i = 0;
    while i < bytes.len() {
        // Field 0x73 = SigningPubKey
        if bytes[i] == 0x73 && i + 1 < bytes.len() {
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

        // Field 0x74 = TxnSignature
        if bytes[i] == 0x74 && i + 1 < bytes.len() {
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

        // Field 0x7C = MemoData or 0x7D = MemoFormat/MemoType
        if (bytes[i] == 0x7C || bytes[i] == 0x7D) && i + 1 < bytes.len() {
            i += 1;
            let len = bytes[i] as usize;
            i += 1;
            if i + len <= bytes.len() {
                // Take the longer one (challenge is longer than "Auth")
                if len > memo_data.len() {
                    memo_data = bytes[i..i + len].to_vec();
                    log::debug!("Found MemoData: {} bytes", memo_data.len());
                }
                i += len;
            }
            continue;
        }

        i += 1;
    }

    log::debug!(
        "Field extraction complete: pubkey={} bytes, signature={} bytes, memo={} bytes",
        signing_pubkey.len(),
        txn_signature.len(),
        memo_data.len()
    );

    if !memo_data.is_empty() {
        if let Ok(memo_str) = String::from_utf8(memo_data.clone()) {
            log::debug!("MemoData (decoded): {}", memo_str);
        }
    }

    Ok(TransactionFields {
        signing_pubkey,
        txn_signature,
        memo_data,
    })
}
