//! Wallet provider implementations.
//!
//! This module contains implementations for different wallet types. Each wallet
//! provider is behind a feature flag, allowing you to compile only the wallets you need.
//!
//! # Available Wallets
//!
//! - **[`xaman`]** - Xaman Wallet (XRPL SignIn) - requires `xaman` feature
//! - **[`web3auth`]** - Web3Auth wallet - requires `web3auth` feature
//!
//! # Features
//!
//! Enable specific wallets by setting features in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! # All wallets (default)
//! wallet-signature-verify = "0.1"
//!
//! # Only Xaman
//! wallet-signature-verify = { version = "0.1", default-features = false, features = ["xaman"] }
//!
//! # Only Web3Auth
//! wallet-signature-verify = { version = "0.1", default-features = false, features = ["web3auth"] }
//! ```

pub mod provider;
pub mod registry;

#[cfg(feature = "web3auth")]
#[cfg_attr(docsrs, doc(cfg(feature = "web3auth")))]
pub mod web3auth;

#[cfg(feature = "xaman")]
#[cfg_attr(docsrs, doc(cfg(feature = "xaman")))]
pub mod xaman;

#[cfg(feature = "bifrost")]
#[cfg_attr(docsrs, doc(cfg(feature = "bifrost")))]
pub mod bifrost;

pub use provider::{VerificationInput, WalletProvider};
pub use registry::{get_wallet_provider, WalletType};

#[cfg(feature = "web3auth")]
#[cfg_attr(docsrs, doc(cfg(feature = "web3auth")))]
pub use web3auth::Web3AuthProvider;

#[cfg(feature = "xaman")]
#[cfg_attr(docsrs, doc(cfg(feature = "xaman")))]
pub use xaman::XamanProvider;

#[cfg(feature = "bifrost")]
#[cfg_attr(docsrs, doc(cfg(feature = "bifrost")))]
pub use bifrost::BifrostProvider;
