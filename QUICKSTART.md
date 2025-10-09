# ⚡ Quick Start - Super Simple Usage

## 🚀 1. Compile the Binary

```bash
cargo build --release
```

The binary will be at: `./target/release/wallet-signature-verify`

---

## ✅ 2. Use in 1 Command

### Syntax:
```bash
./target/release/wallet-signature-verify \
  --wallet <type> \
  --signature <hex> \
  --address <rAddress> \
  --challenge <string>
```

### Xaman Example:
```bash
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "732102DB..." \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "example.com:1234567890:..."
```

### Web3Auth Example:
```bash
./target/release/wallet-signature-verify \
  --wallet web3auth \
  --signature "3045022100D69..." \
  --address "rTestAddr789012345xrpLTest567890abc" \
  --challenge "example.com:1234567891:..."
```

---

## 📊 3. Check the Result

```bash
# Save exit code
./target/release/wallet-signature-verify ...
EXIT_CODE=$?

# Check
if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Valid - Authenticate user"
else
    echo "❌ Invalid - Reject"
fi
```

| Exit Code | Meaning |
|-----------|---------|
| **0** | ✅ **VALID** - All checks passed |
| **1** | ❌ **INVALID** - One or more checks failed |
| **2** | ⚠️ **ERROR** - Invalid arguments |

---

## 🐍 4. Python Integration

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
        return True  # Valid
    except:
        return False  # Invalid

# Use
if verify("xaman", sig, addr, ch):
    print("✅ Authenticated!")
```

---

## 🟢 5. Node.js Integration

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
    return true;  // Valid
  } catch {
    return false;  // Invalid
  }
}

// Use
if (verify('xaman', sig, addr, ch)) {
  console.log('✅ Authenticated!');
}
```

---

## 🔥 Quick Test

Paste this in the terminal to test:

```bash
# Xaman (should return exit code 0)
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "7321ED9434799FED...TRUNCATED...E1F1" \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890"

echo "Exit code: $?"
```

---

## 🦀 6. Rust Integration

Use directly as a library in your project:

```rust
// Cargo.toml
[dependencies]
wallet-signature-verify = { path = "../wallet-signature-verify" }

// main.rs
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    let input = VerificationInput {
        signature_data: sig,
        expected_address: addr,
        challenge: Some(ch),
    };

    let provider = get_wallet_provider(WalletType::Xaman);
    let result = provider.verify(&input)?;

    if result.is_valid() {
        println!("✅ Authenticated!");
    }
    Ok(())
}
```

**See complete example**: `cargo run --example verify_lib`

---

## 🆘 Help

```bash
# See supported wallets
./target/release/wallet-signature-verify --help

# Debug mode (more details)
DEBUG=1 ./target/release/wallet-signature-verify ...
```

---

## 🎯 Summary

### As CLI Binary:
1. **Compile**: `cargo build --release`
2. **Execute**: `./target/release/wallet-signature-verify --wallet xaman --signature <sig> --address <addr> --challenge <ch>`
3. **Check**: `echo $?` (0 = valid, 1 = invalid)
4. **Integrate**: Use in Python/Node/Bash via subprocess

### As Rust Library:
1. **Add**: `wallet-signature-verify = { path = "..." }` in Cargo.toml
2. **Import**: `use wallet_signature_verify::wallets::*;`
3. **Use**: `get_wallet_provider(WalletType::Xaman).verify(&input)?`
4. **Test**: `cargo run --example verify_lib`

**Done! That's it! 🚀**
