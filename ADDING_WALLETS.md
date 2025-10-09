# 🆕 How to Add Support for New Wallets

This guide shows how to add support for a new wallet in **3 simple steps**.

## 📋 Step 1: Add the Wallet to Registry

Edit `src/wallets/registry.rs`:

```rust
// 1. Add the new wallet to enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletType {
    Xaman,
    Web3Auth,
    MetaMask,  // 👈 ADD HERE
}

// 2. Add string parsing
impl WalletType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "xaman" | "xumm" => Ok(WalletType::Xaman),
            "web3auth" => Ok(WalletType::Web3Auth),
            "metamask" => Ok(WalletType::MetaMask),  // 👈 ADD HERE
            _ => Err(format!("Wallet '{}' is not supported", s)),
        }
    }

    // 3. Add to supported wallets list
    pub fn supported_wallets() -> Vec<&'static str> {
        vec!["xaman", "web3auth", "metamask"]  // 👈 ADD HERE
    }
}

// 4. Add to Display
impl fmt::Display for WalletType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletType::Xaman => write!(f, "Xaman"),
            WalletType::Web3Auth => write!(f, "Web3Auth"),
            WalletType::MetaMask => write!(f, "MetaMask"),  // 👈 ADD HERE
        }
    }
}

// 5. Add to provider factory
pub fn get_wallet_provider(wallet_type: WalletType) -> Box<dyn WalletProvider> {
    match wallet_type {
        WalletType::Xaman => Box::new(XamanProvider),
        WalletType::Web3Auth => Box::new(Web3AuthProvider),
        WalletType::MetaMask => Box::new(MetaMaskProvider),  // 👈 ADD HERE
    }
}
```

---

## 📋 Step 2: Create the Provider

Create a new file `src/wallets/metamask.rs`:

```rust
use super::provider::{VerificationInput, WalletProvider};
use crate::types::VerificationResult;

/// Provider for MetaMask
pub struct MetaMaskProvider;

impl WalletProvider for MetaMaskProvider {
    fn name(&self) -> &str {
        "MetaMask"
    }

    fn description(&self) -> &str {
        "MetaMask - Ethereum wallet with XRPL support"
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        // TODO: Add MetaMask-specific validations

        if input.signature_data.is_empty() {
            return Err(anyhow::anyhow!("MetaMask: empty signature_data"));
        }

        if input.challenge.is_none() {
            return Err(anyhow::anyhow!("MetaMask: challenge is required"));
        }

        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        // TODO: Implement MetaMask verification logic

        // Basic example:
        let challenge = input.challenge.as_ref().unwrap();

        // ... your verification logic here ...

        Ok(VerificationResult {
            address_valid: true,
            challenge_valid: true,
            signature_valid: true,
            derived_address: input.expected_address.clone(),
            found_challenge: Some(challenge.clone()),
        })
    }
}
```

---

## 📋 Step 3: Register the Module

Edit `src/wallets/mod.rs`:

```rust
pub mod provider;
pub mod registry;
pub mod web3auth;
pub mod xaman;
pub mod metamask;  // 👈 ADD HERE

pub use provider::{VerificationInput, WalletProvider};
pub use registry::{get_wallet_provider, WalletType};
```

And in `src/wallets/registry.rs`, add the import:

```rust
use super::{
    web3auth::Web3AuthProvider,
    xaman::XamanProvider,
    metamask::MetaMaskProvider,  // 👈 ADD HERE
};
```

---

## ✅ Done!

Now you can use your new wallet:

```bash
cargo run --release -- \
  --wallet metamask \
  --signature <sig_data> \
  --address <xrp_address> \
  --challenge <challenge_string>
```

---

## 📚 Reference of Existing Providers

### Xaman (src/wallets/xaman.rs)
- **Input**: Complete XRPL hex blob (`response.hex`)
- **Verification**: Extracts fields from blob, reconstructs unsigned blob, verifies ECDSA/Ed25519 signature
- **Challenge**: Optional (extracted from MemoData)

### Web3Auth (src/wallets/web3auth.rs)
- **Input**: Hexadecimal DER signature
- **Verification**: Public key recovery (tries 4 recovery IDs), derives address, verifies signature
- **Challenge**: Required

---

## 🎯 Implementation Tips

### 1. Understand the signature format
```rust
fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
    // Validate minimum length
    if input.signature_data.len() < 64 {
        return Err(anyhow::anyhow!("Signature too short"));
    }

    // Validate format (hex, base64, etc)
    if !input.signature_data.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow::anyhow!("Must be hexadecimal"));
    }

    Ok(())
}
```

### 2. Recover/Derive the public key
```rust
// Option A: Public key is included
let pubkey = extract_pubkey_from_signature(&input.signature_data)?;

// Option B: Use public key recovery (like Web3Auth)
let pubkey_candidates = recover_pubkey(&signature, &message_hash);
```

### 3. Derive the XRPL address
```rust
use crate::crypto::account_id_from_pubkey;

let account_id = account_id_from_pubkey(&pubkey);
let derived_address = ripple_address_codec::encode_account_id(&account_id);

if derived_address != input.expected_address {
    return Err(anyhow::anyhow!("Address doesn't match"));
}
```

### 4. Verify the cryptographic signature
```rust
use crate::crypto::{sha512half, verify_signature};

// Hash the challenge
let message_hash = sha512half(challenge.as_bytes());

// Verify signature
let signature_valid = verify_signature(&pubkey, &signature_bytes, &message_hash);
```

---

## 🧪 Testing Your New Wallet

1. **Compile:**
```bash
cargo build --release
```

2. **Test valid:**
```bash
./target/release/wallet-signature-verify \
  --wallet metamask \
  --signature <valid_sig> \
  --address <valid_addr> \
  --challenge <challenge>
```

3. **Test invalid:**
```bash
# Should fail with clear error
./target/release/wallet-signature-verify \
  --wallet metamask \
  --signature "invalid" \
  --address <addr> \
  --challenge <challenge>
```

4. **Test without wallet:**
```bash
# Should list all supported wallets
./target/release/wallet-signature-verify --help
```

---

## 🔍 Debug

Use `DEBUG=1` to see verification details:

```bash
DEBUG=1 cargo run --release -- \
  --wallet metamask \
  --signature <sig> \
  --address <addr> \
  --challenge <challenge>
```

Add prints to your provider:

```rust
if std::env::var("DEBUG").is_ok() {
    println!("   [MetaMask] Verifying signature...");
    println!("   [MetaMask] Signature: {}", input.signature_data);
    println!("   [MetaMask] Challenge: {:?}", input.challenge);
}
```

---

## ✨ Complete Example: Phantom Wallet

See a complete implementation example at:
`examples/phantom_wallet_example.md` (TODO)

---

## 🆘 Help

If you have questions:
1. See existing providers (`xaman.rs`, `web3auth.rs`)
2. Open an issue on GitHub
3. Consult XRPL documentation

---

## 📝 Checklist

- [ ] Add wallet to `WalletType` enum
- [ ] Implement parsing in `from_str()`
- [ ] Add to `supported_wallets()`
- [ ] Implement `Display` trait
- [ ] Add to `get_wallet_provider()`
- [ ] Create file `src/wallets/yourwallet.rs`
- [ ] Implement `WalletProvider` trait
- [ ] Add module in `mod.rs`
- [ ] Test with valid signature
- [ ] Test with invalid signature
- [ ] Update README.md
- [ ] Commit and push!
