//! Bifrost Wallet Provider
//!
//! This module provides signature verification for Bifrost and other EVM-compatible wallets.
//! It supports Ethereum-style signatures (secp256k1) with EIP-191 message signing.

mod provider;
mod core;

pub use provider::BifrostProvider;
