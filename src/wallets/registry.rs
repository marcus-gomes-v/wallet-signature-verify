use super::provider::WalletProvider;
use std::fmt;

#[cfg(feature = "xaman")]
use super::XamanProvider;

#[cfg(feature = "web3auth")]
use super::Web3AuthProvider;

#[cfg(feature = "wallet_connect")]
use super::WalletConnectProvider;

/// Supported wallet types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalletType {
    #[cfg(feature = "xaman")]
    Xaman,
    #[cfg(feature = "web3auth")]
    Web3Auth,
    #[cfg(feature = "wallet_connect")]
    WalletConnect,
}

impl WalletType {
    /// Parse string to WalletType
    ///
    /// Note: We use a custom `from_str` instead of implementing `FromStr` trait
    /// to return a more descriptive error message with supported wallets.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            #[cfg(feature = "xaman")]
            "xaman" | "xumm" => Ok(WalletType::Xaman),
            #[cfg(feature = "web3auth")]
            "web3auth" => Ok(WalletType::Web3Auth),
            #[cfg(feature = "wallet_connect")]
            "wallet_connect" | "walletconnect" => Ok(WalletType::WalletConnect),
            _ => Err(format!("Wallet '{}' is not supported or not enabled", s)),
        }
    }

    /// List all supported wallets (only those enabled via features)
    #[allow(clippy::vec_init_then_push)] // Needed due to conditional compilation
    pub fn supported_wallets() -> Vec<&'static str> {
        let mut wallets = Vec::new();

        #[cfg(feature = "xaman")]
        wallets.push("xaman");

        #[cfg(feature = "web3auth")]
        wallets.push("web3auth");

        #[cfg(feature = "wallet_connect")]
        wallets.push("wallet_connect");

        wallets
    }
}

impl fmt::Display for WalletType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "xaman")]
            WalletType::Xaman => write!(f, "Xaman"),
            #[cfg(feature = "web3auth")]
            WalletType::Web3Auth => write!(f, "Web3Auth"),
            #[cfg(feature = "wallet_connect")]
            WalletType::WalletConnect => write!(f, "WalletConnect"),
        }
    }
}

/// Returns the correct provider for the wallet
pub fn get_wallet_provider(wallet_type: WalletType) -> Box<dyn WalletProvider> {
    match wallet_type {
        #[cfg(feature = "xaman")]
        WalletType::Xaman => Box::new(XamanProvider),
        #[cfg(feature = "web3auth")]
        WalletType::Web3Auth => Box::new(Web3AuthProvider),
        #[cfg(feature = "wallet_connect")]
        WalletType::WalletConnect => Box::new(WalletConnectProvider),
    }
}

/// Shows error message with supported wallets
pub fn unsupported_wallet_error(wallet: &str) -> String {
    format!(
        "❌ Wallet '{}' is not supported.\n\n✅ Supported wallets:\n{}",
        wallet,
        WalletType::supported_wallets()
            .iter()
            .map(|w| format!("  - {}", w))
            .collect::<Vec<_>>()
            .join("\n")
    )
}
