# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2025-10-10

### Added
- 🧪 **35 comprehensive Bitcoin-grade tests**
  - 17 crypto unit tests (SHA-512Half, RIPEMD-160, ECDSA, Ed25519)
  - 9 Xaman integration tests with real signatures
  - 8 Web3Auth integration tests
- ✅ Test coverage reporting with `cargo-llvm-cov`
- 📊 Codecov.io integration for coverage badges
- 📚 Complete testing documentation (TESTING.md, tests/README.md)
- 🚀 Test runner script (`test-ci.sh`) for local CI simulation
- 🛡️ Security attack tests (replay attacks, tampering, spoofing)

### Changed
- 📦 **MSRV updated to Rust 1.85+** (from 1.80)
  - Required for `edition2024` dependencies
- 🔧 Improved CI/CD with GitHub Actions
  - Runs tests on every push/PR
  - Checks formatting (rustfmt)
  - Runs linter (clippy)
  - Generates code coverage
- 📖 Updated README with test coverage section
- 🎯 CI now tests only on `stable` Rust (simplified)

### Fixed
- ✅ All clippy warnings addressed
- ✅ Code formatting standardized with `rustfmt`
- 🐛 GitHub Actions workflow fixed for correct Rust version

### Security
- ✅ Comprehensive security testing suite
- ✅ Verified protection against:
  - Signature tampering
  - Replay attacks
  - Address spoofing
  - Malformed data injection

## [0.1.3] - 2025-10-10

### Added
- Improved documentation for docs.rs
- Feature badges in README

### Changed
- Better Cargo.toml configuration for docs.rs

## [0.1.2] - 2025-10-10

### Added
- GitHub Actions build status badge

## [0.1.1] - 2025-10-10

### Added
- Initial release
- Xaman wallet support (XRPL SignIn)
- Web3Auth support (secp256k1 signatures)
- CLI binary
- Library API with features

[0.1.4]: https://github.com/marcus-gomes-v/wallet-signature-verify/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/marcus-gomes-v/wallet-signature-verify/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/marcus-gomes-v/wallet-signature-verify/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/marcus-gomes-v/wallet-signature-verify/releases/tag/v0.1.1
