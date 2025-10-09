# 🔐 Wallet Signature Verifier

Universal library and CLI for cryptographic verification of wallet signatures using **challenge-response authentication**.

## 🎯 What does it do?

This tool implements a **secure authentication system** based on 4 components:

1. 📋 **Challenge** - A unique message/payload to be signed (e.g., "domain:timestamp:uuid:action:address")
2. 📍 **Address** - The wallet address that signed the challenge
3. ✍️ **Signature** - The cryptographic signature (hex or DER format)
4. 🎯 **Wallet Type** - Which wallet was used (Xaman, Web3Auth, etc.)

**The verifier receives these 4 components and validates that the address actually signed the challenge.**

### Currently supports:
- 🦊 **Xaman Wallet** (XRPL SignIn transactions)
- 🌐 **Web3Auth** (secp256k1 raw signatures)
- 🔧 **Extensible architecture** to easily add any wallet from any blockchain

### 3 security layers:
1. ✅ **Cryptographic Verification** (ECDSA/Ed25519) - Proves the signature is valid
2. ✅ **Address Derivation** - Proves the public key belongs to the expected address
3. ✅ **Challenge Verification** - Prevents replay attacks by verifying the unique challenge

---

## 📦 Quick Start

### As Binary (CLI)
```bash
# 1. Compile
cargo build --release

# 2. Execute with: wallet type, signature, address, challenge
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "<hex_blob>" \
  --address "<wallet_address>" \
  --challenge "<unique_challenge>"

# 3. Check exit code
echo $?  # 0 = valid, 1 = invalid
```

### As Rust Library
```rust
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    // The 4 components:
    let input = VerificationInput {
        signature_data: signature,    // The signature hex
        expected_address: address,    // The wallet address
        challenge: Some(challenge),   // The unique challenge
    };

    // Select the wallet type
    let provider = get_wallet_provider(WalletType::Xaman);

    // Verify that the address signed the challenge
    let result = provider.verify(&input)?;

    if result.is_valid() {
        println!("✅ Authenticated! Address {} signed the challenge", address);
    }
    Ok(())
}
```

**📚 See more:** [LIBRARY_USAGE.md](./LIBRARY_USAGE.md)

---

## 🚀 Installation

### Prerequisites
- Rust 1.70+ ([rustup.rs](https://rustup.rs/))

### Build
```bash
cargo build --release
# Binary: ./target/release/wallet-signature-verify
```

---

## 📖 Usage

### CLI - Syntax

```bash
./target/release/wallet-signature-verify \
  --wallet <type> \
  --signature <signature_hex> \
  --address <wallet_address> \
  --challenge <challenge_string>
```

**Parameters:**
- `--wallet` - Wallet type: `xaman` or `web3auth`
- `--signature` - Signature hex (full blob for Xaman, DER for Web3Auth)
- `--address` - Wallet address that signed (e.g., rAddress for XRPL, 0x for Ethereum)
- `--challenge` - Unique challenge string that was signed

**Exit Codes:**
- `0` = ✅ Valid (address signed the challenge - all checks passed)
- `1` = ❌ Invalid (signature verification failed)
- `2` = ⚠️  Usage error (invalid arguments or unsupported wallet)

### Quick Examples

**Xaman (XRPL):**
```bash
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "732102DB48115142..." \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "example.com:1234567890:..."
```

**Web3Auth (secp256k1):**
```bash
./target/release/wallet-signature-verify \
  --wallet web3auth \
  --signature "3045022100D69B7099..." \
  --address "rTestAddr789012345xrpLTest567890abc" \
  --challenge "example.com:1234567891:..."
```

**📚 See more examples:**
- [QUICKSTART.md](./QUICKSTART.md) - 5-minute quick start
- [USAGE.md](./USAGE.md) - Examples in Python/Node/Rust
- [examples/](./examples/) - Ready-to-use scripts

---

## 🦀 Usage as Rust Library

### Installation

Add to your `Cargo.toml`:

```toml
# All wallets (default)
[dependencies]
wallet-signature-verify = "0.1"

# Or from path
[dependencies]
wallet-signature-verify = { path = "../wallet-signature-verify" }
```

### 🎯 Optional Features

Choose which wallets to include to reduce compilation time and dependencies:

```toml
# Only Xaman wallet
[dependencies]
wallet-signature-verify = { version = "0.1", default-features = false, features = ["xaman"] }

# Only Web3Auth wallet
[dependencies]
wallet-signature-verify = { version = "0.1", default-features = false, features = ["web3auth"] }

# Multiple specific wallets
[dependencies]
wallet-signature-verify = { version = "0.1", default-features = false, features = ["xaman", "web3auth"] }

# All wallets (same as default)
[dependencies]
wallet-signature-verify = { version = "0.1", features = ["all-wallets"] }
```

**Available Features:**
- `xaman` - Xaman Wallet (XRPL SignIn) support
- `web3auth` - Web3Auth wallet support
- `cli` - CLI binary with logging (for binary only)
- `all-wallets` - Convenience feature for all wallets
- **default** = `["xaman", "web3auth", "cli"]`

**Benefits of selective features:**
- ✅ Faster compile times
- ✅ Smaller binary size
- ✅ Only include what you need

### Basic Example
```rust
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

// Challenge-response authentication:
// 1. Client receives a challenge from server
// 2. Client signs the challenge with their wallet
// 3. Client sends: address + signature + challenge
// 4. Server verifies that the address signed the challenge

let input = VerificationInput {
    signature_data: sig,          // Signature from wallet
    expected_address: addr,       // Wallet address
    challenge: Some(challenge),   // Original challenge
};

let provider = get_wallet_provider(WalletType::Xaman);
let result = provider.verify(&input)?;

if result.is_valid() {
    // ✅ Verified: The address actually signed this challenge
    // Now you can authenticate the user!
}
```

### Complete Example
```bash
cargo run --example verify_lib
```

**📚 Complete documentation:** [LIBRARY_USAGE.md](./LIBRARY_USAGE.md)

---

## 🔌 Backend Integration

### Python
```python
import subprocess

def verify(wallet, signature, address, challenge):
    try:
        subprocess.run([
            "./target/release/wallet-signature-verify",
            "--wallet", wallet,
            "--signature", signature,
            "--address", address,
            "--challenge", challenge
        ], check=True, capture_output=True)
        return True  # Address signed the challenge
    except:
        return False  # Verification failed

if verify("xaman", sig, addr, ch):
    print("✅ Authenticated!")
```

### Node.js
```javascript
const { execSync } = require('child_process');

function verify(wallet, signature, address, challenge) {
  try {
    execSync(
      `./target/release/wallet-signature-verify ` +
      `--wallet ${wallet} ` +
      `--signature "${signature}" ` +
      `--address "${address}" ` +
      `--challenge "${challenge}"`,
      { stdio: 'pipe' }
    );
    return true;  // Address signed the challenge
  } catch {
    return false;  // Verification failed
  }
}

if (verify('xaman', sig, addr, ch)) {
  console.log('✅ Authenticated!');
}
```

**📚 See complete scripts:** [examples/](./examples/)

---

## 🧪 Testing

### Xaman (valid - exit code 0)
```bash
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "7321ED9434799FED...TRUNCATED...E1F1" \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890"
```

### Web3Auth (valid - exit code 0)
```bash
./target/release/wallet-signature-verify \
  --wallet web3auth \
  --signature "3045022100ABC123...TRUNCATED...7890" \
  --address "rTestAddr789012345xrpLTest567890abc" \
  --challenge "example.com:1234567891:87654321-dcba-4321-dcba-987654321cba:login:rTestAddr789012345xrpLTest567890abc"
```

### Rust (as library)
```bash
cargo run --example verify_lib
```

---

## 🔒 Security - Challenge-Response Authentication

**How it works:**

1. **Server generates unique challenge:**
   ```
   domain:timestamp:uuid:action:address
   Example: example.com:1234567890:f0a3a280-...:login:rAddress...
   ```

2. **Client signs the challenge with their wallet**

3. **Client sends 3 things to server:**
   - Wallet address
   - Signature
   - The original challenge

4. **Server verifies:**
   - ✅ Cryptographic signature is valid
   - ✅ Public key derives to the claimed address
   - ✅ Challenge matches exactly (prevents replay attacks)

**Recommended Challenge Format:**
```
{domain}:{timestamp}:{uuid}:{action}:{address}
```

Example:
```
example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890
```

Where:
- `domain` - Your domain (prevents phishing)
- `timestamp` - Unix timestamp (allows expiration)
- `uuid` - Unique ID (prevents replay)
- `action` - Requested action (login, approve, etc)
- `address` - User address (binding to specific wallet)

**Supported Algorithms:**
- ✅ **secp256k1 ECDSA** (Ethereum, XRPL, Bitcoin, etc)
- ✅ **Ed25519** (Solana, XRPL ED accounts, etc)

---

## 🌍 Multi-Blockchain Support

The architecture is **blockchain-agnostic**. Currently supports XRPL wallets, but can be extended to:

- ✅ **Ethereum** (MetaMask, WalletConnect, etc)
- ✅ **Solana** (Phantom, Solflare, etc)
- ✅ **Bitcoin** (Any secp256k1 wallet)
- ✅ **Cosmos** (Keplr, etc)
- ✅ **Any blockchain** with ECDSA or Ed25519 signatures

To add a new wallet, simply implement the `WalletProvider` trait!

---

## 🤝 Contributing

### Adding a New Wallet

The architecture is extensible! To add support for a new wallet:

1. Copy the template: `src/wallets/WALLET_TEMPLATE/`
2. Implement the `WalletProvider` trait
3. Register in `WalletType` enum
4. Test and submit PR!

**📚 Complete guide:** [ADDING_WALLETS.md](./ADDING_WALLETS.md)

### Contributing Code

```bash
# 1. Fork the repository
# 2. Clone your fork
git clone https://github.com/your-username/wallet-signature-verify

# 3. Create a branch
git checkout -b feature/my-feature

# 4. Make your changes and test
cargo test
cargo build --release

# 5. Commit and push
git commit -m "feat: add support for WalletX"
git push origin feature/my-feature

# 6. Open a Pull Request
```

**📚 See more:** [CONTRIBUTING.md](./CONTRIBUTING.md)

---

## 📚 Documentation

| File | Description |
|------|-------------|
| [QUICKSTART.md](./QUICKSTART.md) | 5-minute quick start |
| [USAGE.md](./USAGE.md) | Examples in Python/Node/Rust/Bash |
| [LIBRARY_USAGE.md](./LIBRARY_USAGE.md) | Complete guide for using as Rust library |
| [CONTRIBUTING.md](./CONTRIBUTING.md) | How to contribute to the project |
| [ADDING_WALLETS.md](./ADDING_WALLETS.md) | How to add support for new wallets |
| [examples/](./examples/) | Ready-to-use scripts |

---

## 🛠️ Debug & Logging

Control log verbosity using the `RUST_LOG` environment variable:

```bash
# Info level (default) - shows important verification steps
cargo run --release -- --wallet xaman --signature "..." --address "..." --challenge "..."

# Debug level - shows all verification details
RUST_LOG=debug cargo run --release -- --wallet xaman --signature "..." --address "..." --challenge "..."

# Warn level - shows only warnings and errors
RUST_LOG=warn cargo run --release -- --wallet xaman --signature "..." --address "..." --challenge "..."

# Error level - shows only errors
RUST_LOG=error cargo run --release -- --wallet xaman --signature "..." --address "..." --challenge "..."
```

**Debug level shows:**
- Reconstructed unsigned blob (hex)
- Extracted public key and signature
- Calculated SHA-512Half digest
- ECDSA/Ed25519 verification steps
- Public key recovery attempts (Web3Auth)
- Field extraction from XRPL blobs

**Info level shows:**
- Wallet provider name
- Address verification result
- Challenge verification result
- Signature verification result

**Warn/Error levels:**
- Only warnings and errors (minimal output)

---

## 🐛 Troubleshooting

**Unsupported wallet:**
```bash
./target/release/wallet-signature-verify --help
# See list of supported wallets
```

**Invalid signature (exit code 1):**
- Check if the signature hex is complete
- Confirm the challenge matches exactly
- Ensure the address is correct
- Test with example data from [QUICKSTART.md](./QUICKSTART.md)

**Arguments error (exit code 2):**
- Use `--help` to see correct syntax
- Verify all 4 arguments are provided

---

## 📄 License

MIT License - Use freely in commercial and open source projects.

---

## 🙏 Credits

Developed for the blockchain community with a focus on security, extensibility, and ease of use.

**Challenge-response authentication:** A secure way to prove wallet ownership without exposing private keys.
