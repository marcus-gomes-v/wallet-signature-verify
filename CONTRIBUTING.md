# 🤝 Contribution Guide - Wallet Provider Standard

This guide ensures that **all wallets follow the same standard**, making contributions and maintenance easier.

## 📁 Standard Structure (MANDATORY)

**ALL wallets MUST follow this structure:**

```
src/wallets/
├── provider.rs              # ✅ Trait (don't touch)
├── registry.rs              # ✅ Factory (add wallet here)
├── WALLET_TEMPLATE/         # 📝 Template to copy
│   ├── mod.rs
│   ├── provider.rs
│   └── README.md
├── xaman/                   # ✅ Example: Simple wallet
│   ├── mod.rs
│   └── provider.rs
└── web3auth/                # ✅ Example: Complex wallet
    ├── mod.rs
    ├── provider.rs
    ├── core.rs              # Additional logic
    └── recover.rs           # Additional logic
```

---

## 🆕 Adding a New Wallet (5 steps)

### **Step 1: Copy Template**

```bash
cp -r src/wallets/WALLET_TEMPLATE src/wallets/metamask
```

### **Step 2: Rename in Code**

**`src/wallets/metamask/mod.rs`:**
```rust
pub mod provider;

pub use provider::MetaMaskProvider;  // 👈 Change here
```

**`src/wallets/metamask/provider.rs`:**
```rust
use super::super::provider::{VerificationInput, WalletProvider};
use crate::types::VerificationResult;

pub struct MetaMaskProvider;  // 👈 Change here

impl WalletProvider for MetaMaskProvider {
    fn name(&self) -> &str {
        "MetaMask"  // 👈 Wallet name
    }

    fn description(&self) -> &str {
        "MetaMask - Ethereum wallet with XRPL support"  // 👈 Description
    }

    fn validate_input(&self, input: &VerificationInput) -> anyhow::Result<()> {
        // 👇 Your validations
        if input.signature_data.is_empty() {
            return Err(anyhow::anyhow!("MetaMask: empty signature_data"));
        }
        Ok(())
    }

    fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
        self.validate_input(input)?;

        // 👇 Your verification logic

        Ok(VerificationResult {
            address_valid: true,
            challenge_valid: true,
            signature_valid: true,
            derived_address: input.expected_address.clone(),
            found_challenge: input.challenge.clone(),
        })
    }
}
```

### **Step 3: Register in Enum**

**`src/wallets/registry.rs`:**

```rust
// 1️⃣ Add to enum
pub enum WalletType {
    Xaman,
    Web3Auth,
    MetaMask,  // 👈 ADD HERE
}

// 2️⃣ Add to parser
impl WalletType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "xaman" | "xumm" => Ok(WalletType::Xaman),
            "web3auth" => Ok(WalletType::Web3Auth),
            "metamask" => Ok(WalletType::MetaMask),  // 👈 ADD HERE
            _ => Err(format!("Wallet '{}' is not supported", s)),
        }
    }

    // 3️⃣ Add to list
    pub fn supported_wallets() -> Vec<&'static str> {
        vec!["xaman", "web3auth", "metamask"]  // 👈 ADD HERE
    }
}

// 4️⃣ Add to Display
impl fmt::Display for WalletType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalletType::Xaman => write!(f, "Xaman"),
            WalletType::Web3Auth => write!(f, "Web3Auth"),
            WalletType::MetaMask => write!(f, "MetaMask"),  // 👈 ADD HERE
        }
    }
}

// 5️⃣ Add to factory
pub fn get_wallet_provider(wallet_type: WalletType) -> Box<dyn WalletProvider> {
    match wallet_type {
        WalletType::Xaman => Box::new(XamanProvider),
        WalletType::Web3Auth => Box::new(Web3AuthProvider),
        WalletType::MetaMask => Box::new(MetaMaskProvider),  // 👈 ADD HERE
    }
}

// 6️⃣ Add to import
use super::{Web3AuthProvider, XamanProvider, MetaMaskProvider};  // 👈 ADD HERE
```

### **Step 4: Export the Module**

**`src/wallets/mod.rs`:**
```rust
pub mod provider;
pub mod registry;
pub mod web3auth;
pub mod xaman;
pub mod metamask;  // 👈 ADD HERE

pub use provider::{VerificationInput, WalletProvider};
pub use registry::{get_wallet_provider, WalletType};
pub use web3auth::Web3AuthProvider;
pub use xaman::XamanProvider;
pub use metamask::MetaMaskProvider;  // 👈 ADD HERE
```

### **Step 5: Test**

```bash
# Compile
cargo build --release

# Test
cargo run --release -- \
  --wallet metamask \
  --signature <sig> \
  --address <addr> \
  --challenge <challenge>
```

---

## ✅ Contribution Checklist

Before making a Pull Request, verify:

- [ ] **Structure**: Created folder `src/wallets/my_wallet/`
- [ ] **Files**: Has `mod.rs` and `provider.rs`
- [ ] **Provider**: Implements `WalletProvider` trait
- [ ] **Validation**: Implemented `validate_input()`
- [ ] **Verification**: Implemented `verify()`
- [ ] **Registry**: Added to `WalletType` enum (6 places)
- [ ] **Export**: Added in `wallets/mod.rs`
- [ ] **Compiled**: `cargo build --release` without errors
- [ ] **Tested**: Valid signature returns exit code 0
- [ ] **Tested**: Invalid signature returns exit code 1
- [ ] **Documented**: Commented complex code

---

## 📚 Reference Examples

### Simple Wallet (Xaman)
```
xaman/
├── mod.rs           # Simple export
└── provider.rs      # Everything in 1 file
```

Use this pattern if your wallet:
- Doesn't need complex logic
- All logic fits in ~50 lines

### Complex Wallet (Web3Auth)
```
web3auth/
├── mod.rs           # Multiple exports
├── provider.rs      # Provider implementation
├── core.rs          # Verification logic
└── recover.rs       # Public key recovery
```

Use this pattern if your wallet:
- Needs public key recovery
- Has complex verification logic
- Multiple files make maintenance easier

---

## 🔍 Code Standards

### Imports (ALWAYS like this)
```rust
use super::super::provider::{VerificationInput, WalletProvider};
use crate::types::VerificationResult;
// ... other specific imports
```

### Provider Name (ALWAYS `NameProvider`)
```rust
pub struct XamanProvider;      // ✅ Correct
pub struct XamanWallet;         // ❌ Wrong
pub struct Xaman;               // ❌ Wrong
```

### Validation (ALWAYS with wallet prefix)
```rust
return Err(anyhow::anyhow!("Xaman: empty signature_data"));  // ✅ Correct
return Err(anyhow::anyhow!("empty signature_data"));         // ❌ Wrong
```

### Documentation (ALWAYS in English)
```rust
/// Provider for Xaman Wallet (XRPL SignIn)  // ✅ Correct
/// Provider para Xaman Wallet                // ❌ Wrong
```

---

## 🐛 Debug

Add debug prints to your provider:

```rust
fn verify(&self, input: &VerificationInput) -> anyhow::Result<VerificationResult> {
    if std::env::var("DEBUG").is_ok() {
        println!("🔍 [MetaMask] Starting verification");
        println!("   Signature: {}", input.signature_data);
        println!("   Address: {}", input.expected_address);
    }

    // ... your logic
}
```

Test with:
```bash
DEBUG=1 cargo run --release -- --wallet metamask ...
```

---

## ❌ What NOT to do

### ❌ Don't create loose file
```
wallets/
├── metamask.rs  # ❌ WRONG - Not standardized
```

### ❌ Don't change existing structure
```rust
// ❌ WRONG - Don't change the trait
pub trait WalletProvider {
    fn my_custom_method(&self);  // ❌ Don't add here
}
```

### ❌ Don't mix languages
```rust
fn name(&self) -> &str {
    "MetaMask"  // ✅ Name OK in English
}

fn description(&self) -> &str {
    "MetaMask - Ethereum wallet"  // ✅ Description in English
}
```

---

## 🆘 Need Help?

1. See existing wallets: `xaman/` (simple) or `web3auth/` (complex)
2. Copy the `WALLET_TEMPLATE/`
3. Open an issue on GitHub
4. Consult `ADDING_WALLETS.md` for detailed examples

---

## 🎉 After Contributing

1. Make descriptive commit:
   ```bash
   git add src/wallets/metamask/
   git commit -m "feat: add MetaMask wallet support"
   ```

2. Test locally:
   ```bash
   cargo test
   cargo run --release -- --wallet metamask ...
   ```

3. Open Pull Request with:
   - Clear description of wallet
   - Usage examples
   - Tests performed

---

**Thank you for contributing! 🚀**
