use hex::FromHex;

/// Reconstructs the unsigned blob by removing the signature
/// but keeping the TxnSignature field empty
pub fn reconstruct_unsigned_blob(signed_hex: &str) -> anyhow::Result<Vec<u8>> {
    log::debug!("Reconstructing unsigned blob from signed transaction");
    log::debug!("Signed hex length: {} chars", signed_hex.len());

    let full_bytes = Vec::from_hex(signed_hex)?;
    log::debug!("Decoded {} bytes from hex", full_bytes.len());

    let mut unsigned_blob = Vec::new();
    let mut i = 0;
    let mut signature_removed = false;

    while i < full_bytes.len() {
        // Look for SigningPubKey (0x73)
        if full_bytes[i] == 0x73 && i + 1 < full_bytes.len() {
            let pk_len = full_bytes[i + 1] as usize;
            if i + 2 + pk_len <= full_bytes.len() {
                // Copy SigningPubKey field
                log::debug!("Copying SigningPubKey field ({} bytes)", pk_len);
                unsigned_blob.extend_from_slice(&full_bytes[i..i + 2 + pk_len]);
                i += 2 + pk_len;

                // Check for TxnSignature (0x74) immediately after
                if i < full_bytes.len() && full_bytes[i] == 0x74 && i + 1 < full_bytes.len() {
                    let sig_len = full_bytes[i + 1] as usize;
                    if i + 2 + sig_len <= full_bytes.len() {
                        // Skip TxnSignature completely (don't add it to unsigned blob)
                        log::debug!("Removing TxnSignature field ({} bytes)", sig_len);
                        signature_removed = true;
                        i += 2 + sig_len;
                        continue;
                    }
                }
                continue;
            }
        }
        unsigned_blob.push(full_bytes[i]);
        i += 1;
    }

    if !signature_removed {
        log::warn!("TxnSignature field not found in blob");
    }

    log::debug!("Unsigned blob reconstructed: {} bytes", unsigned_blob.len());

    // Add XRPL signing prefix
    let mut prefixed = Vec::with_capacity(4 + unsigned_blob.len());
    prefixed.extend_from_slice(&[0x53, 0x54, 0x58, 0x00]); // "STX\0"
    prefixed.extend_from_slice(&unsigned_blob);

    log::debug!("Added XRPL signing prefix (STX\\0)");
    log::debug!("Final prefixed blob: {} bytes", prefixed.len());

    Ok(prefixed)
}
