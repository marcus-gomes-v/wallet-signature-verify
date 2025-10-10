# How Signature Verification Works

This document explains in detail how our wallet signature verification library works, specifically focusing on the cryptographic security and mathematical principles behind it.

## Table of Contents

- [Fundamental Concepts](#fundamental-concepts)
- [What We Receive](#what-we-receive)
- [Step-by-Step Verification Process](#step-by-step-verification-process)
- [The Math Behind Signature Verification](#the-math-behind-signature-verification)
- [Why Attacks Are Impossible](#why-attacks-are-impossible)
- [Complete Verification Flow](#complete-verification-flow)
- [Common Questions for Meetings](#common-questions-for-meetings)
- [Key Points to Remember](#key-points-to-remember)

## Fundamental Concepts

### What is a Digital Signature?

A digital signature is a mathematical proof that:
1. A specific private key signed a specific message
2. The signature cannot be forged without knowing the private key
3. The message cannot be altered without invalidating the signature

This is based on **elliptic curve cryptography** (ECC), specifically:
- **ECDSA secp256k1** (same as Bitcoin/Ethereum) - most common for XRPL
- **Ed25519** (alternative signature scheme) - also supported by XRPL

### What We Receive

Our library receives 3 critical pieces of information:

1. **Signature Hex Blob** - A hexadecimal string containing:
   - The signed transaction (XRPL SignIn format)
   - Public key (33 bytes for secp256k1, 32 bytes for Ed25519)
   - Signature (typically 71-73 bytes for ECDSA DER format)
   - Challenge text embedded in MemoData field

2. **Wallet Address** - The XRPL address (starts with 'r'), e.g., `rExampleAddress123`

3. **Challenge** - The expected challenge string, e.g.:
   ```
   example.com:1760079290:57e06102-c0c8-4cf8-be97-530c2515a55d:login:rExampleAddress123
   ```

## Step-by-Step Verification Process

### Step 1: Extract Fields from Hex Blob

**File**: `src/parser/mod.rs`

```rust
pub fn extract_fields(signed_hex: &str) -> anyhow::Result<TransactionFields>
```

The hex blob is actually a serialized XRPL transaction in binary format. We parse this using XRPL's binary codec to extract:

- **SigningPubKey** (field code `73`) - The public key used to sign
- **TxnSignature** (field code `74`) - The actual signature bytes
- **Account** (field code `81`) - The account ID (20 bytes)
- **MemoData** (field code `7D`) - The challenge text in hex format

**Example**:
```
732102DB48...  → Field 73 (SigningPubKey) = 02DB48...
7447304502...  → Field 74 (TxnSignature) = 304502...
8114368...     → Field 81 (Account) = 368...
7C044175...    → Field 7C (MemoData) = "example.com:..."
```

### Step 2: Verify Address Derivation

**File**: `src/crypto/hash.rs:35-49`

```rust
pub fn account_id_from_pubkey(pubkey: &[u8]) -> [u8; 20]
```

This is a **one-way function** that proves the public key matches the address.

**Process**:
1. Take the public key (33 bytes)
2. Apply SHA-256 hash → 32 bytes
3. Apply RIPEMD-160 hash → 20 bytes = **Account ID**
4. Encode Account ID using Base58Check → XRPL Address (starts with 'r')

**Mathematical Security**:
- SHA-256 and RIPEMD-160 are **one-way hash functions**
- Impossible to reverse: you cannot derive the public key from the address
- If we derive an XRPL address from the public key in the signature, and it matches the expected address, we know the public key is legitimate

**Why This Matters**:
If someone tries to fake a signature with a different public key, the derived address will be completely different. This proves the public key belongs to the wallet address.

### Step 3: Verify Challenge

**File**: `src/lib.rs:145-162`

```rust
fn verify_challenge(
    fields: &TransactionFields,
    expected_challenge: Option<&str>,
) -> (bool, Option<String>)
```

**Process**:
1. Extract MemoData from the hex blob (field `7C`)
2. Decode from hex to UTF-8 string
3. Compare exact string match with expected challenge

**Security Purpose**:
- Prevents **replay attacks**: Each challenge contains:
  - Domain name (`example.com`)
  - Unix timestamp (`1760079290`)
  - Unique UUID (`57e06102-c0c8-4cf8-be97-530c2515a55d`)
  - Action (`login`)
  - Address (XRPL address starting with 'r')

- If an attacker steals a signed signature, they cannot reuse it because:
  - The timestamp will be expired
  - The UUID will be different
  - The domain will be wrong for their attack

### Step 4: Cryptographic Signature Verification (MOST IMPORTANT)

This is where the **real cryptographic security** happens.

#### Step 4a: Reconstruct Unsigned Blob

**File**: `src/parser/mod.rs:124-155`

```rust
pub fn reconstruct_unsigned_blob(signed_hex: &str) -> anyhow::Result<Vec<u8>>
```

**Process**:
1. Parse the signed hex blob
2. **Remove** the TxnSignature field (field `74`)
3. Prepend with signing prefix: `535458 00` (STX\0 in ASCII)
4. Result: The exact bytes that were originally signed

**Why This Is Critical**:
The signature was created by signing the **unsigned transaction bytes**. We must reconstruct those exact bytes to verify the signature.

**Example**:
```
Original unsigned: 535458 00 732102DB48... 8114368... 7C044175... F1
                   ^^^^^^^^^ ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                   Prefix    Transaction fields WITHOUT signature
```

#### Step 4b: Hash the Unsigned Blob

**File**: `src/crypto/hash.rs:10-18`

```rust
pub fn sha512half(data: &[u8]) -> [u8; 32]
```

**Process**:
1. Apply SHA-512 to the unsigned blob → 64 bytes
2. Take the first 32 bytes → **Message Digest**

**Why SHA-512Half**:
This is the XRPL standard hashing method. The 32-byte digest is what was actually signed by the private key.

**Mathematical Property**:
- SHA-512 is a **cryptographic hash function**
- Any tiny change to the input produces a completely different output
- Impossible to find two different inputs with the same output (collision resistance)
- Impossible to reverse: cannot derive input from output

#### Step 4c: Verify the Signature

**File**: `src/crypto/verify.rs:10-58`

```rust
pub fn verify_signature(pubkey: &[u8], signature: &[u8], digest: &[u8; 32]) -> bool
```

This is the **core cryptographic verification**.

**For ECDSA secp256k1** (most common):

```rust
use secp256k1::{ecdsa::Signature, Message, PublicKey, Secp256k1};

let secp = Secp256k1::verification_only();
let pubkey = PublicKey::from_slice(pubkey)?;
let signature = Signature::from_der(signature)?;
let message = Message::from_digest_slice(digest)?;

secp.verify_ecdsa(&message, &signature, &pubkey)
```

**What This Does**:
Uses the `secp256k1` library (the same library used by Bitcoin Core) to verify that:
```
signature = sign(digest, private_key)
```

**The Mathematics** (explained below in detail)

## The Math Behind Signature Verification

### Elliptic Curve Digital Signature Algorithm (ECDSA)

ECDSA is based on elliptic curve mathematics. Here's what happens:

#### When Signing (done by the wallet, e.g., Xaman):

1. **Private Key**: A secret random number `d` (256 bits)
2. **Public Key**: A point on the elliptic curve `Q = d × G`
   - `G` is the generator point (fixed for secp256k1)
   - `×` is elliptic curve point multiplication
   - This operation is **one-way**: given `Q`, it's impossible to find `d` (discrete logarithm problem)

3. **Signing Process**:
   - Generate random nonce `k`
   - Calculate point `R = k × G`
   - Extract `r` = x-coordinate of `R`
   - Calculate `s = k⁻¹ × (hash + r × d) mod n`
   - Signature = `(r, s)`

#### When Verifying (done by our library):

Given: Public Key `Q`, Signature `(r, s)`, Message Hash `hash`

1. Calculate `u₁ = hash × s⁻¹ mod n`
2. Calculate `u₂ = r × s⁻¹ mod n`
3. Calculate point `R' = u₁ × G + u₂ × Q`
4. **Signature is valid if**: `r == x-coordinate of R'`

**Why This Works Mathematically**:

```
R' = u₁ × G + u₂ × Q
   = (hash × s⁻¹) × G + (r × s⁻¹) × Q
   = (hash × s⁻¹) × G + (r × s⁻¹) × (d × G)      [because Q = d × G]
   = (hash × s⁻¹ + r × s⁻¹ × d) × G
   = s⁻¹ × (hash + r × d) × G
   = s⁻¹ × (k × s) × G                           [because s = k⁻¹ × (hash + r × d)]
   = k × G
   = R                                           [the original point used in signing]
```

**Critical Security**:

Without knowing the private key `d`, it's impossible to create a valid signature because:
- You need to calculate `s = k⁻¹ × (hash + r × d) mod n`
- Without `d`, you cannot compute this correctly
- The discrete logarithm problem makes it impossible to derive `d` from `Q`
- With 256-bit keys, there are 2²⁵⁶ possible values (more than atoms in the universe)

### Why Tampering Is Impossible

**Scenario**: Attacker tries to modify the challenge

1. Original signature was created for: `hash1 = SHA512Half(unsigned_blob1)`
2. Attacker modifies challenge: `hash2 = SHA512Half(unsigned_blob2)`
3. When we verify with `hash2`, the equation fails:
   ```
   R' = (hash2 × s⁻¹) × G + (r × s⁻¹) × Q
      ≠ R
   ```
4. Because `s` was calculated using `hash1`, not `hash2`
5. To create a valid signature for `hash2`, attacker needs the private key

## Why Attacks Are Impossible

### Attack 1: Replay Attack (Reusing a Valid Signature)

**What attacker tries**:
- Captures a valid signature
- Tries to use it to authenticate as the same user on a different request

**Why it fails**:
1. Each challenge contains a **timestamp** and **unique UUID**
2. Our server validates:
   ```rust
   let expected_challenge = format!(
       "{}:{}:{}:{}:{}",
       domain, current_timestamp, fresh_uuid, action, address
   );
   ```
3. The old signature contains the **old challenge** in MemoData
4. Challenge verification fails: `old_challenge ≠ expected_challenge`
5. Even though the signature is cryptographically valid, the challenge is wrong

**Code**: `src/lib.rs:119`

### Attack 2: Tampering with the Challenge

**What attacker tries**:
- Takes a valid signed blob
- Modifies the MemoData field to change the challenge
- Tries to use it

**Why it fails**:
1. Modifying MemoData changes the unsigned blob
2. The hash changes: `SHA512Half(modified_blob) ≠ SHA512Half(original_blob)`
3. The signature was created for the **original hash**, not the modified one
4. Signature verification fails mathematically (see ECDSA math above)

**Proof from our tests**:
```rust
// tests/xaman_integration_tests.rs
#[test]
fn test_xaman_wrong_challenge_rejected() {
    let result = verify_xrpl_signin(
        valid_signature,
        valid_address,
        Some("ATTACKER:trying:to:replay:attack")  // Wrong challenge
    ).expect("Verification should not error");

    assert!(!result.challenge_valid);  // ✓ Test passes
}
```

### Attack 3: Address Spoofing

**What attacker tries**:
- Takes a valid signature from address A
- Claims it's from address B
- Tries to authenticate as address B

**Why it fails**:
1. The signature contains the **public key**
2. We derive the address from the public key:
   ```rust
   let account_id = SHA256(RIPEMD160(pubkey));
   let address = base58_encode(account_id);
   ```
3. Derived address will be **address A**, not address B
4. Address verification fails: `derived_address ≠ expected_address`

**Code**: `src/lib.rs:108-116`

**Proof from our tests**:
```rust
// tests/xaman_integration_tests.rs
#[test]
fn test_xaman_wrong_address_rejected() {
    let result = verify_xrpl_signin(
        valid_signature,
        "rATTACKERAddressXXXXXXXXXXXXXXXXXXXXXXX",  // Wrong address
        Some(valid_challenge)
    ).expect("Verification should not error");

    assert!(!result.address_valid);  // ✓ Test passes
}
```

### Attack 4: Signature Forgery

**What attacker tries**:
- Generate a fake signature without knowing the private key
- Try to authenticate

**Why it fails**:
1. To create a valid signature `(r, s)` for a message hash `h`:
   ```
   s = k⁻¹ × (h + r × d) mod n
   ```
2. This requires knowing the **private key `d`**
3. The private key is never transmitted or stored in our system
4. **Breaking this requires solving the discrete logarithm problem**:
   - Find `d` given `Q = d × G`
   - With 256-bit secp256k1, this requires ~2¹²⁸ operations
   - With current technology: ~10⁷⁵ years to break one key
   - More secure than mining all possible Bitcoin blocks

**Proof from our tests**:
```rust
// tests/xaman_integration_tests.rs
#[test]
fn test_xaman_tampered_signature_rejected() {
    let tampered_hex = "...modified...";  // Modified signature bytes

    let result = verify_xrpl_signin(
        tampered_hex,
        valid_address,
        Some(valid_challenge)
    ).expect("Verification should not error");

    assert!(!result.signature_valid);  // ✓ Test passes
}
```

**Additional proof**:
```rust
// src/crypto/verify.rs tests
#[test]
fn test_verify_signature_wrong_digest() {
    let wrong_digest = [0u8; 32];  // All zeros
    let result = verify_signature(&pubkey, &signature, &wrong_digest);
    assert!(!result);  // ✓ Signature fails with wrong digest
}
```

### Attack 5: Public Key Substitution

**What attacker tries**:
- Generate their own key pair (attacker_pubkey, attacker_privkey)
- Sign the challenge with attacker_privkey
- Include attacker_pubkey in the signed blob
- Try to authenticate as victim's address

**Why it fails**:
1. We derive the address from the **public key in the signature**:
   ```rust
   let account_id = account_id_from_pubkey(&fields.signing_pubkey);
   let derived_address = ripple_address_codec::encode_account_id(&account_id);
   ```
2. `attacker_pubkey` will derive to `attacker_address`, not victim's address
3. Address verification fails: `attacker_address ≠ victim_address`
4. The attacker can only prove they own their own address, not the victim's

**Mathematical Guarantee**:
- Each public key uniquely maps to exactly one address (one-way hash function)
- Collision probability: ~1 in 2¹⁶⁰ (more than atoms in the observable universe)

## Complete Verification Flow

```
┌─────────────────────────────────────────────────────────────┐
│                    INPUT FROM USER                          │
│  • Signature Hex Blob                                       │
│  • Expected Address (e.g., rExample...)                     │
│  • Expected Challenge (e.g., example.com:timestamp:...)     │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│           STEP 1: Parse Signed Hex Blob                     │
│  parser::extract_fields(signed_hex)                         │
│  ✓ Extract SigningPubKey (field 73)                         │
│  ✓ Extract TxnSignature (field 74)                          │
│  ✓ Extract Account (field 81)                               │
│  ✓ Extract MemoData (field 7C) = challenge text             │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│         STEP 2: Verify Address Derivation                   │
│  crypto::account_id_from_pubkey(signing_pubkey)             │
│  ✓ SHA-256(pubkey) → 32 bytes                               │
│  ✓ RIPEMD-160(hash) → 20 bytes = account_id                 │
│  ✓ Base58Check(account_id) → XRPL address                   │
│  ✓ derived_address == expected_address?                     │
│  ❌ If NO: address_valid = false (FAIL)                     │
│  ✓ If YES: Continue...                                      │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│           STEP 3: Verify Challenge                          │
│  ✓ Decode MemoData from hex → UTF-8 string                  │
│  ✓ found_challenge == expected_challenge?                   │
│  ❌ If NO: challenge_valid = false (FAIL)                   │
│  ✓ If YES: Continue...                                      │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│   STEP 4: Cryptographic Signature Verification              │
│                                                              │
│  4a. Reconstruct Unsigned Blob                              │
│      parser::reconstruct_unsigned_blob(signed_hex)          │
│      ✓ Remove TxnSignature field (74)                       │
│      ✓ Prepend "STX\0" prefix (535458 00)                   │
│      → unsigned_blob                                         │
│                                                              │
│  4b. Hash Unsigned Blob                                     │
│      crypto::sha512half(unsigned_blob)                      │
│      ✓ SHA-512(unsigned_blob) → 64 bytes                    │
│      ✓ Take first 32 bytes → digest                         │
│                                                              │
│  4c. Verify ECDSA Signature                                 │
│      crypto::verify_signature(pubkey, signature, digest)    │
│      ✓ Use secp256k1 library (Bitcoin Core standard)        │
│      ✓ Verify: R' = (h × s⁻¹) × G + (r × s⁻¹) × Q          │
│      ✓ Check: r == x-coordinate of R'                       │
│      ❌ If NO: signature_valid = false (FAIL)               │
│      ✓ If YES: signature_valid = true (PASS)                │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        ▼
┌─────────────────────────────────────────────────────────────┐
│                    FINAL RESULT                             │
│  VerificationResult {                                       │
│    address_valid: bool,     // Address matches pubkey       │
│    challenge_valid: bool,   // Challenge matches MemoData   │
│    signature_valid: bool,   // Crypto signature valid       │
│    derived_address: String, // The actual derived address   │
│    found_challenge: Option<String>, // The actual challenge │
│  }                                                           │
│                                                              │
│  ✓ ALL THREE MUST BE TRUE FOR SUCCESSFUL AUTHENTICATION     │
└─────────────────────────────────────────────────────────────┘
```

## References

- **XRPL Binary Format**: https://xrpl.org/serialization.html
- **ECDSA secp256k1**: https://en.bitcoin.it/wiki/Secp256k1
- **secp256k1 library**: https://github.com/rust-bitcoin/rust-secp256k1
- **Ed25519**: https://ed25519.cr.yp.to/
- **Our test suite**: `tests/xaman_integration_tests.rs`, `tests/web3auth_integration_tests.rs`
- **Crypto implementation**: `src/crypto/verify.rs`, `src/crypto/hash.rs`

---

**Last Updated**: 2025-10-10
**Version**: 0.1.5
