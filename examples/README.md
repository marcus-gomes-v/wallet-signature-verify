# 📚 Usage Examples

This folder contains practical examples of how to use the signature verifier in different languages.

## 📁 Files

- **`verify.sh`** - Bash script for simple verification
- **`verify.py`** - Python script with reusable function
- **`verify.js`** - Node.js script with reusable function
- **`verify_lib.rs`** - Example of usage as Rust library

---

## 🚀 Quick Usage

### Bash
```bash
./examples/verify.sh xaman <signature> <address> <challenge>
```

### Python
```bash
python3 examples/verify.py xaman <signature> <address> <challenge>
```

### Node.js
```bash
node examples/verify.js xaman <signature> <address> <challenge>
```

### Rust (as library)
```bash
cargo run --example verify_lib
```

---

## ✅ Quick Test

### Xaman (valid)
```bash
./examples/verify.sh xaman \
  "7321ED9434799FED...TRUNCATED...E1F1" \
  "rExampleAddr123456789xrpL1234567890" \
  "example.com:1234567890:12345678-abcd-1234-abcd-123456789abc:login:rExampleAddr123456789xrpL1234567890"
```

### Web3Auth (valid)
```bash
./examples/verify.sh web3auth \
  "3045022100ABC123...TRUNCATED...7890" \
  "rTestAddr789012345xrpLTest567890abc" \
  "example.com:1234567891:87654321-dcba-4321-dcba-987654321cba:login:rTestAddr789012345xrpLTest567890abc"
```

---

## 📖 Integration in Your Code

### Python
```python
from verify import verify_signature

result = verify_signature("xaman", signature, address, challenge)
if result["valid"]:
    print("✅ Authenticated!")
```

### Node.js
```javascript
const { verifySignature } = require('./verify.js');

const result = verifySignature('xaman', signature, address, challenge);
if (result.valid) {
  console.log('✅ Authenticated!');
}
```

### Rust
```rust
use wallet_signature_verify::{
    wallets::{get_wallet_provider, WalletType, VerificationInput},
};

fn main() -> anyhow::Result<()> {
    let input = VerificationInput {
        signature_data: signature,
        expected_address: address,
        challenge: Some(challenge),
    };

    let provider = get_wallet_provider(WalletType::Xaman);
    let result = provider.verify(&input)?;

    if result.is_valid() {
        println!("✅ Authenticated!");
    }
    Ok(())
}
```

**In your Cargo.toml:**
```toml
[dependencies]
wallet-signature-verify = { path = "../wallet-signature-verify" }
```

---

## 🔑 Exit Codes

| Code | Meaning |
|------|---------|
| `0` | ✅ Valid authentication |
| `1` | ❌ Authentication failed |
| `2` | ⚠️  Usage error |
