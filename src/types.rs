/// Fields extracted from a signed XRPL transaction
#[derive(Debug, Clone)]
pub struct TransactionFields {
    pub signing_pubkey: Vec<u8>,
    pub txn_signature: Vec<u8>,
    pub memo_data: Vec<u8>,
}

/// Authentication verification result
#[derive(Debug)]
pub struct VerificationResult {
    pub address_valid: bool,
    pub challenge_valid: bool,
    pub signature_valid: bool,
    pub derived_address: String,
    pub found_challenge: Option<String>,
}

impl VerificationResult {
    pub fn is_valid(&self) -> bool {
        self.address_valid && self.challenge_valid && self.signature_valid
    }
}
