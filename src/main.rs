use std::env;
use wallet_signature_verify::{
    output::print_verification_result,
    wallets::{
        get_wallet_provider, registry::unsupported_wallet_error, VerificationInput, WalletType,
    },
};

fn print_usage(program_name: &str) {
    eprintln!("Wallet Signature Verifier - Challenge-Response Authentication");
    eprintln!();
    eprintln!("Usage:");
    eprintln!(
        "  {} --wallet <wallet_type> --signature <sig> --address <addr> --challenge <challenge>",
        program_name
    );
    eprintln!();
    eprintln!("Environment Variables:");
    eprintln!("  RUST_LOG=debug    Show detailed verification steps");
    eprintln!("  RUST_LOG=info     Show general information (default)");
    eprintln!("  RUST_LOG=warn     Show only warnings and errors");
    eprintln!("  RUST_LOG=error    Show only errors");
    eprintln!();
    eprintln!("Arguments:");
    eprintln!("  --wallet <type>        Wallet type (xaman, web3auth)");
    eprintln!("  --signature <data>     Signature data (hex blob for Xaman, DER sig for Web3Auth)");
    eprintln!("  --address <addr>       Expected XRP address (rAddress format)");
    eprintln!(
        "  --challenge <str>      Challenge string (optional for Xaman, required for Web3Auth)"
    );
    eprintln!();
    eprintln!("Supported Wallets:");
    for wallet in WalletType::supported_wallets() {
        let provider = get_wallet_provider(WalletType::from_str(wallet).unwrap());
        eprintln!("  - {}: {}", provider.name(), provider.description());
    }
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  # Xaman");
    eprintln!(
        "  {} --wallet xaman --signature <hex_blob> --address <addr> --challenge <str>",
        program_name
    );
    eprintln!();
    eprintln!("  # Web3Auth");
    eprintln!(
        "  {} --wallet web3auth --signature <der_hex> --address <addr> --challenge <str>",
        program_name
    );
}

fn main() -> anyhow::Result<()> {
    // Initialize logger (only available with cli feature)
    #[cfg(feature = "cli")]
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        std::process::exit(2);
    }

    log::debug!("Starting wallet signature verification");
    log::debug!("Arguments: {:?}", args);

    // Parse arguments
    let mut wallet_type: Option<String> = None;
    let mut signature_data: Option<String> = None;
    let mut expected_address: Option<String> = None;
    let mut challenge: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--wallet" | "-w" => {
                if i + 1 < args.len() {
                    wallet_type = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --wallet requires a value");
                    std::process::exit(2);
                }
            }
            "--signature" | "-s" => {
                if i + 1 < args.len() {
                    signature_data = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --signature requires a value");
                    std::process::exit(2);
                }
            }
            "--address" | "-a" => {
                if i + 1 < args.len() {
                    expected_address = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --address requires a value");
                    std::process::exit(2);
                }
            }
            "--challenge" | "-c" => {
                if i + 1 < args.len() {
                    challenge = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    eprintln!("Error: --challenge requires a value");
                    std::process::exit(2);
                }
            }
            "--help" | "-h" => {
                print_usage(&args[0]);
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error: Unknown argument '{}'", args[i]);
                eprintln!();
                print_usage(&args[0]);
                std::process::exit(2);
            }
        }
    }

    // Validate required arguments
    let wallet_type = wallet_type.ok_or_else(|| {
        eprintln!("Error: --wallet is required");
        eprintln!();
        print_usage(&args[0]);
        anyhow::anyhow!("Missing --wallet")
    })?;

    let signature_data = signature_data.ok_or_else(|| {
        eprintln!("Error: --signature is required");
        anyhow::anyhow!("Missing --signature")
    })?;

    let expected_address = expected_address.ok_or_else(|| {
        eprintln!("Error: --address is required");
        anyhow::anyhow!("Missing --address")
    })?;

    // Parse wallet type
    let wallet_type = match WalletType::from_str(&wallet_type) {
        Ok(wt) => wt,
        Err(_) => {
            eprintln!("{}", unsupported_wallet_error(&wallet_type));
            std::process::exit(2);
        }
    };

    // Get the appropriate provider
    let provider = get_wallet_provider(wallet_type);

    log::info!("Using wallet provider: {}", provider.name());
    log::info!("Provider description: {}", provider.description());
    log::debug!("Expected address: {}", expected_address);
    log::debug!("Challenge: {:?}", challenge);

    println!();
    println!("🔐 Verifying with: {}", provider.name());
    println!("   {}", provider.description());
    println!();

    // Create verification input
    let input = VerificationInput {
        signature_data: signature_data.clone(),
        expected_address: expected_address.clone(),
        challenge: challenge.clone(),
    };

    log::debug!("Signature data length: {} bytes", signature_data.len());
    log::debug!("Starting verification process...");

    // Verify signature
    let result = provider.verify(&input)?;

    log::debug!("Verification completed");
    log::debug!("Address valid: {}", result.address_valid);
    log::debug!("Challenge valid: {}", result.challenge_valid);
    log::debug!("Signature valid: {}", result.signature_valid);

    // Print results
    print_verification_result(&result, &expected_address, challenge.as_deref());

    // Exit with appropriate code
    if result.is_valid() {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
