# 🦀 Using wallet-signature-verify as a Rust Library

This guide shows how to integrate the XRPL signature verifier directly into your Rust project.

---

## 📦 Installation

### Option 1: Local Path
```toml
# Cargo.toml
[dependencies]
wallet-signature-verify = { path = "../wallet-signature-verify" }
```

### Option 2: Git (when published)
```toml
# Cargo.toml
[dependencies]
wallet-signature-verify = { git = "https://github.com/your-repo/wallet-signature-verify" }
```

### Option 3: crates.io (when published)
```toml
# Cargo.toml
[dependencies]
wallet-signature-verify = "0.1.0"
```

---

## 🚀 Basic Usage

### Simple Example

```rust
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    // 1. Create input
    let input = VerificationInput {
        signature_data: "732102DB48115142459C05AA0D26F3752ADC...".to_string(),
        expected_address: "rExampleAddr123456789xrpL1234567890".to_string(),
        challenge: Some("example.com:1234567890:...".to_string()),
    };

    // 2. Get wallet provider
    let provider = get_wallet_provider(WalletType::Xaman);

    // 3. Verify
    let result = provider.verify(&input)?;

    // 4. Check result
    if result.is_valid() {
        println!("✅ Authenticated!");
        // Create session, JWT, etc
    } else {
        println!("❌ Invalid signature");
    }

    Ok(())
}
```

---

## 📚 Complete API

### WalletType Enum

```rust
pub enum WalletType {
    Xaman,      // Xaman Wallet (XRPL SignIn)
    Web3Auth,   // Web3Auth (secp256k1)
}

impl WalletType {
    // Parse from string (case-insensitive)
    pub fn from_str(s: &str) -> Result<Self, String>

    // List of supported wallets
    pub fn supported_wallets() -> Vec<&'static str>
}
```

**Example:**
```rust
let wallet = WalletType::from_str("xaman")?;
let wallets = WalletType::supported_wallets(); // ["xaman", "web3auth"]
```

---

### VerificationInput Struct

```rust
pub struct VerificationInput {
    pub signature_data: String,      // Signature hex or blob
    pub expected_address: String,    // Expected XRP address
    pub challenge: Option<String>,   // Challenge to validate
}
```

**Example:**
```rust
let input = VerificationInput {
    signature_data: hex_signature,
    expected_address: "rAddress...".to_string(),
    challenge: Some("domain:timestamp:uuid:action:address".to_string()),
};
```

---

### VerificationResult Struct

```rust
pub struct VerificationResult {
    pub address_valid: bool,        // Derived address == expected?
    pub challenge_valid: bool,      // Found challenge == expected?
    pub signature_valid: bool,      // Valid ECDSA/Ed25519 signature?
    pub derived_address: String,    // Address derived from public key
    pub found_challenge: Option<String>, // Challenge found in memo
}

impl VerificationResult {
    // Check if ALL checks passed
    pub fn is_valid(&self) -> bool
}
```

**Example:**
```rust
let result = provider.verify(&input)?;

println!("Valid address: {}", result.address_valid);
println!("Valid challenge: {}", result.challenge_valid);
println!("Valid signature: {}", result.signature_valid);
println!("Derived address: {}", result.derived_address);

if result.is_valid() {
    // All checks passed
}
```

---

### WalletProvider Trait

```rust
pub trait WalletProvider: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult>;
    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()>;
}
```

**Factory Function:**
```rust
pub fn get_wallet_provider(wallet_type: WalletType) -> Box<dyn WalletProvider>
```

---

## 🎯 Common Use Cases

### 1. Web/API Authentication

```rust
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn authenticate_user(
    wallet_type: &str,
    signature: String,
    address: String,
    challenge: String,
) -> anyhow::Result<bool> {
    let wallet = WalletType::from_str(wallet_type)
        .map_err(|e| anyhow::anyhow!(e))?;

    let provider = get_wallet_provider(wallet);

    let input = VerificationInput {
        signature_data: signature,
        expected_address: address,
        challenge: Some(challenge),
    };

    let result = provider.verify(&input)?;
    Ok(result.is_valid())
}

// Use in HTTP endpoint
match authenticate_user(&wallet_type, signature, address, challenge) {
    Ok(true) => {
        // Create session
        let session_token = create_jwt_token(&address)?;
        Ok(Json(LoginResponse { token: session_token }))
    }
    Ok(false) => Err(StatusCode::UNAUTHORIZED),
    Err(e) => {
        log::error!("Auth error: {}", e);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

---

### 2. Dynamic Wallet Type Parsing

```rust
use wallet_signature_verify::wallets::{get_wallet_provider, registry::WalletType};

fn verify_from_user_input(
    wallet_str: &str,
    signature: String,
    address: String,
    challenge: Option<String>,
) -> anyhow::Result<bool> {
    // Parse wallet type (from user input)
    let wallet_type = match WalletType::from_str(wallet_str) {
        Ok(wt) => wt,
        Err(e) => {
            eprintln!("❌ {}", e);
            eprintln!("💡 Supported wallets: {:?}", WalletType::supported_wallets());
            return Err(anyhow::anyhow!(e));
        }
    };

    let provider = get_wallet_provider(wallet_type);

    let input = VerificationInput {
        signature_data: signature,
        expected_address: address,
        challenge,
    };

    let result = provider.verify(&input)?;
    Ok(result.is_valid())
}
```

---

### 3. Multi-Wallet Support

```rust
use wallet_signature_verify::wallets::{get_wallet_provider, WalletType, VerificationInput};

fn verify_any_wallet(
    signature: String,
    address: String,
    challenge: Option<String>,
) -> anyhow::Result<(bool, WalletType)> {
    // Try all wallet types
    let wallet_types = vec![WalletType::Xaman, WalletType::Web3Auth];

    for wallet_type in wallet_types {
        let provider = get_wallet_provider(wallet_type);

        let input = VerificationInput {
            signature_data: signature.clone(),
            expected_address: address.clone(),
            challenge: challenge.clone(),
        };

        // Try to verify
        if let Ok(result) = provider.verify(&input) {
            if result.is_valid() {
                return Ok((true, wallet_type));
            }
        }
    }

    Ok((false, WalletType::Xaman)) // None worked
}

// Use
match verify_any_wallet(signature, address, challenge)? {
    (true, wallet_type) => {
        println!("✅ Authenticated via {:?}", wallet_type);
    }
    (false, _) => {
        println!("❌ No wallet could verify");
    }
}
```

---

### 4. Custom Validation

```rust
use wallet_signature_verify::wallets::{get_wallet_provider, WalletType, VerificationInput};

fn verify_with_custom_checks(
    wallet_type: WalletType,
    input: VerificationInput,
) -> anyhow::Result<VerificationResult> {
    let provider = get_wallet_provider(wallet_type);

    // 1. Validate input first
    provider.validate_input(&input)?;

    // 2. Custom checks
    if input.expected_address.len() != 34 {
        return Err(anyhow::anyhow!("Invalid XRP address"));
    }

    if let Some(ref challenge) = input.challenge {
        if !challenge.contains(':') {
            return Err(anyhow::anyhow!("Malformed challenge"));
        }
    }

    // 3. Verify signature
    let result = provider.verify(&input)?;

    // 4. Additional checks on result
    if result.derived_address != input.expected_address {
        log::warn!(
            "Address mismatch: derived={}, expected={}",
            result.derived_address,
            input.expected_address
        );
    }

    Ok(result)
}
```

---

## 🧪 Testing

### Unit Test

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use wallet_signature_verify::{
        wallets::{get_wallet_provider, WalletType, VerificationInput},
    };

    #[test]
    fn test_xaman_authentication() {
        let input = VerificationInput {
            signature_data: "7321ED9434799FED...TRUNCATED...E1F1".to_string(),
            expected_address: "rExampleAddr123456789xrpL1234567890".to_string(),
            challenge: Some("example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890".to_string()),
        };

        let provider = get_wallet_provider(WalletType::Xaman);
        let result = provider.verify(&input).unwrap();

        assert!(result.is_valid());
        assert_eq!(result.derived_address, "rExampleAddr123456789xrpL1234567890");
    }

    #[test]
    fn test_invalid_wallet_type() {
        let result = WalletType::from_str("invalid_wallet");
        assert!(result.is_err());
    }
}
```

---

## 📝 Complete Example

See the `examples/verify_lib.rs` file for a complete and functional example:

```bash
cargo run --example verify_lib
```

---

## 🔗 See Also

- **QUICKSTART.md** - Usage as CLI binary
- **USAGE.md** - Examples in Python/Node/Bash
- **CONTRIBUTING.md** - How to contribute
- **ADDING_WALLETS.md** - How to add support for new wallets

---

## 🆘 Help

If you encounter problems, see:

1. **API Documentation**: `cargo doc --open`
2. **Functional example**: `cargo run --example verify_lib`
3. **Tests**: `cargo test`
4. **Issues**: GitHub Issues (when published)

---

**Ready to use! 🚀**
