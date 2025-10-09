pub mod provider;
pub mod registry;

#[cfg(feature = "web3auth")]
pub mod web3auth;

#[cfg(feature = "xaman")]
pub mod xaman;

pub use provider::{VerificationInput, WalletProvider};
pub use registry::{get_wallet_provider, WalletType};

#[cfg(feature = "web3auth")]
pub use web3auth::Web3AuthProvider;

#[cfg(feature = "xaman")]
pub use xaman::XamanProvider;
