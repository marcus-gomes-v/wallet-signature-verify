# 🧪 Testing Guide

## Quick Test Commands

```bash
# Run all tests
cargo test

# Run all tests with output
cargo test -- --nocapture

# Run only unit tests (crypto)
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run specific test file
cargo test --test xaman_integration_tests
cargo test --test web3auth_integration_tests
```

## 🚀 Test Like CI (Before Pushing)

Run this script to test everything the CI will run:

```bash
./test-ci.sh
```

Or manually:

```bash
# 1. Check compilation
cargo check --all-features

# 2. Run unit tests
cargo test --lib

# 3. Run integration tests
cargo test --test '*'

# 4. Run all tests
cargo test --all-targets

# 5. Check formatting
cargo fmt --all -- --check

# 6. Run linter
cargo clippy --all-targets --all-features -- -D warnings

# 7. Build release
cargo build --release
```

## 🔧 Fix Issues

### Formatting Issues
```bash
# Auto-fix formatting
cargo fmt --all
```

### Clippy Warnings
```bash
# See all warnings
cargo clippy --all-targets --all-features

# Auto-fix some warnings
cargo clippy --fix --all-targets --all-features
```

## 📊 Test Coverage

```
✅ 35 Total Tests
  - 17 Crypto Unit Tests (hash + signature verification)
  - 9 Xaman Integration Tests (with real signatures)
  - 8 Web3Auth Integration Tests
  - 1 Doc Test

✅ 0 Failures
✅ 100% Pass Rate
```

## 🎯 What Gets Tested

### Cryptographic Functions
- SHA-512Half hashing
- RIPEMD-160 address derivation
- ECDSA secp256k1 signature verification
- Ed25519 signature verification
- Deterministic behavior

### Xaman Wallet
- Valid signature acceptance
- Invalid signature rejection
- Replay attack prevention
- Address spoofing protection
- Tampering detection
- Challenge validation

### Web3Auth
- Public key recovery
- DER signature parsing
- Address format validation
- Deterministic verification

## 🛡️ Security Testing

All tests verify that attacks are **impossible**:

```bash
# Test with wrong challenge (replay attack)
cargo test test_xaman_wrong_challenge_rejected

# Test with tampered signature
cargo test test_xaman_tampered_signature_rejected

# Test with wrong address
cargo test test_xaman_wrong_address_rejected
```

## 📝 Adding New Tests

1. Unit tests go in `src/**/*.rs` within `#[cfg(test)]` modules
2. Integration tests go in `tests/*.rs` files
3. Follow the pattern:
   ```rust
   #[test]
   fn test_descriptive_name() {
       // Arrange
       let input = "test data";

       // Act
       let result = function_under_test(input);

       // Assert
       assert!(result.is_ok());
   }
   ```

## 🔍 Debug Tests

```bash
# Run with debug output
RUST_LOG=debug cargo test -- --nocapture

# Run specific test with debug
RUST_LOG=debug cargo test test_xaman_valid_signature -- --nocapture

# Run tests and show println! output
cargo test -- --nocapture --test-threads=1
```

## 🎓 Test Philosophy

This project follows **Bitcoin-level testing**:

1. ✅ **Don't trust, verify** - Every claim is tested
2. ✅ **Real-world data** - Use actual signatures from wallets
3. ✅ **Attack-driven** - Test what attackers would try
4. ✅ **Cryptographic proof** - Verify math, not just strings
5. ✅ **Zero tolerance** - 100% pass rate required

## 🤖 CI/CD

GitHub Actions automatically runs all tests on:
- Every push to `main`
- Every pull request
- Multiple Rust versions (stable + 1.80 MSRV)

View CI results: https://github.com/marcus-gomes-v/wallet-signature-verify/actions
