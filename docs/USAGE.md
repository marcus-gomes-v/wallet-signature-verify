# 🚀 How to Use - Simple Guide

## 📖 Basic Syntax

```bash
./wallet-signature-verify \
  --wallet <type> \
  --signature <signature_or_hex> \
  --address <xrp_address> \
  --challenge <challenge_string>
```

---

## ✅ Example 1: Xaman Wallet

```bash
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "732102DB48115142459C05AA0D26F3752ADC..." \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890"
```

**Result:**
```
✅ AUTHENTICATION SUCCESSFUL
Exit code: 0
```

---

## ✅ Example 2: Web3Auth

```bash
./target/release/wallet-signature-verify \
  --wallet web3auth \
  --signature "3045022100D69B7099756C3C1CCAA3A388EDC..." \
  --address "rTestAddr789012345xrpLTest567890abc" \
  --challenge "example.com:1234567891:87654321-dcba-4321-dcba-987654321cba:login:rTestAddr789012345xrpLTest567890abc"
```

**Result:**
```
✅ AUTHENTICATION SUCCESSFUL
Exit code: 0
```

---

## 🔴 Example 3: Invalid Signature

```bash
./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "INVALID_SIGNATURE" \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "challenge_string"
```

**Result:**
```
❌ AUTHENTICATION FAILED
Exit code: 1
```

---

## 🐍 Python Example

```python
import subprocess
import json

def verify_signature(wallet_type, signature, address, challenge):
    """
    Verify signature using the binary

    Args:
        wallet_type: "xaman" or "web3auth"
        signature: Signature hex
        address: XRP address (rAddress)
        challenge: Challenge string

    Returns:
        dict: {"valid": bool, "error": str|None}
    """
    try:
        result = subprocess.run(
            [
                "./target/release/wallet-signature-verify",
                "--wallet", wallet_type,
                "--signature", signature,
                "--address", address,
                "--challenge", challenge
            ],
            capture_output=True,
            text=True,
            check=True
        )
        return {"valid": True, "error": None}
    except subprocess.CalledProcessError as e:
        return {"valid": False, "error": e.stderr}

# Usage example
result = verify_signature(
    wallet_type="xaman",
    signature="732102DB48115142459C05AA0D26F3752ADC...",
    address="rExampleAddr123456789xrpL1234567890",
    challenge="example.com:1234567890:..."
)

if result["valid"]:
    print("✅ User authenticated!")
else:
    print(f"❌ Failed: {result['error']}")
```

---

## 🟢 Node.js Example

```javascript
const { execSync } = require('child_process');

function verifySignature(walletType, signature, address, challenge) {
  /**
   * Verify signature using the binary
   *
   * @param {string} walletType - "xaman" or "web3auth"
   * @param {string} signature - Signature hex
   * @param {string} address - XRP address (rAddress)
   * @param {string} challenge - Challenge string
   * @returns {{valid: boolean, error: string|null}}
   */
  try {
    execSync(
      `./target/release/wallet-signature-verify ` +
      `--wallet ${walletType} ` +
      `--signature "${signature}" ` +
      `--address "${address}" ` +
      `--challenge "${challenge}"`,
      { stdio: 'pipe' }
    );
    return { valid: true, error: null };
  } catch (error) {
    return { valid: false, error: error.stderr.toString() };
  }
}

// Usage example
const result = verifySignature(
  'xaman',
  '732102DB48115142459C05AA0D26F3752ADC...',
  'rExampleAddr123456789xrpL1234567890',
  'example.com:1234567890:...'
);

if (result.valid) {
  console.log('✅ User authenticated!');
} else {
  console.log(`❌ Failed: ${result.error}`);
}
```

---

## 🔧 Rust Example (as library)

```rust
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    // Configure input
    let input = VerificationInput {
        signature_data: "732102DB48115142459C05AA0D26F3752ADC...".to_string(),
        expected_address: "rExampleAddr123456789xrpL1234567890".to_string(),
        challenge: Some("example.com:1234567890:...".to_string()),
    };

    // Get correct provider
    let provider = get_wallet_provider(WalletType::Xaman);

    // Verify
    let result = provider.verify(&input)?;

    if result.is_valid() {
        println!("✅ User authenticated!");
    } else {
        println!("❌ Authentication failed");
    }

    Ok(())
}
```

---

## 🔑 Exit Codes

Use exit codes to check the result:

```bash
./target/release/wallet-signature-verify --wallet xaman ...

if [ $? -eq 0 ]; then
    echo "✅ Valid authentication"
elif [ $? -eq 1 ]; then
    echo "❌ Authentication failed"
elif [ $? -eq 2 ]; then
    echo "⚠️  Usage error (invalid arguments)"
fi
```

| Exit Code | Meaning |
|-----------|---------|
| `0` | ✅ Valid authentication (ALL checks passed) |
| `1` | ❌ Authentication failed (one or more checks failed) |
| `2` | ⚠️  Usage error (invalid arguments or unsupported wallet) |

---

## 📝 Supported Wallets

To see the list of supported wallets:

```bash
./target/release/wallet-signature-verify --help
```

Currently:
- ✅ `xaman` (Xaman Wallet - XRPL SignIn)
- ✅ `web3auth` (Web3Auth - secp256k1)

---

## 🐞 Debug Mode

To see detailed verification information:

```bash
DEBUG=1 ./target/release/wallet-signature-verify \
  --wallet xaman \
  --signature "..." \
  --address "..." \
  --challenge "..."
```

This will show:
- Reconstructed unsigned blob
- Extracted public key
- Calculated digest
- ECDSA/Ed25519 verification steps

---

## ⚡ Quick Test

Quick test if it's working:

```bash
# Xaman
cargo run --release -- \
  --wallet xaman \
  --signature "7321ED9434799FED...TRUNCATED...E1F1" \
  --address "rExampleAddr123456789xrpL1234567890" \
  --challenge "example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890"
```

---

## 📦 Build

Before using, compile the binary:

```bash
cargo build --release
```

The binary will be at: `./target/release/wallet-signature-verify`
