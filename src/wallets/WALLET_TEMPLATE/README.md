# 🆕 How to Use This Template

## 1. Copy the folder

```bash
cp -r src/wallets/WALLET_TEMPLATE src/wallets/metamask
```

## 2. Rename in code

In `mod.rs`:
```rust
pub use provider::MetaMaskProvider;  // 👈 Change here
```

In `provider.rs`:
```rust
pub struct MetaMaskProvider;  // 👈 Change here

impl WalletProvider for MetaMaskProvider {
    fn name(&self) -> &str {
        "MetaMask"  // 👈 Change here
    }

    fn description(&self) -> &str {
        "MetaMask - Ethereum wallet with XRPL support"  // 👈 Change here
    }

    // ... implement validate_input and verify
}
```

## 3. Register in registry

In `src/wallets/registry.rs`:

```rust
// 1. Add to enum
pub enum WalletType {
    Xaman,
    Web3Auth,
    MetaMask,  // 👈 Add
}

// 2. Add to from_str
"metamask" => Ok(WalletType::MetaMask),  // 👈 Add

// 3. Add to list
vec!["xaman", "web3auth", "metamask"]  // 👈 Add

// 4. Add to Display
WalletType::MetaMask => write!(f, "MetaMask"),  // 👈 Add

// 5. Add to factory
WalletType::MetaMask => Box::new(MetaMaskProvider),  // 👈 Add
```

## 4. Add the module

In `src/wallets/mod.rs`:

```rust
pub mod metamask;  // 👈 Add

pub use metamask::MetaMaskProvider;  // 👈 Add
```

In `src/wallets/registry.rs` (imports):

```rust
use super::{Web3AuthProvider, XamanProvider, MetaMaskProvider};  // 👈 Add
```

## 5. Compile and test!

```bash
cargo build --release
cargo run --release -- --wallet metamask --signature <sig> --address <addr> --challenge <ch>
```

---

## 📁 Final Structure

```
wallets/
├── provider.rs          # Interface (Trait)
├── registry.rs          # Factory
├── xaman/
│   ├── mod.rs
│   └── provider.rs
├── web3auth/
│   ├── mod.rs
│   ├── provider.rs
│   ├── core.rs
│   └── recover.rs
└── metamask/            # 👈 Your new wallet!
    ├── mod.rs
    └── provider.rs
```
