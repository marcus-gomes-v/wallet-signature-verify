#!/usr/bin/env node
/**
 * 🔐 XRPL Signature Verifier - Node.js Example
 *
 * Usage:
 *   node verify.js xaman <signature> <address> <challenge>
 *   node verify.js web3auth <signature> <address> <challenge>
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

/**
 * Verify XRPL signature using the Rust binary
 *
 * @param {string} walletType - Wallet type ("xaman" or "web3auth")
 * @param {string} signature - Signature hex
 * @param {string} address - XRP address (rAddress format)
 * @param {string} challenge - Challenge string
 * @returns {{valid: boolean, error: string|null, exitCode: number}}
 */
function verifySignature(walletType, signature, address, challenge) {
  const binaryPath = './target/release/wallet-signature-verify';

  // Check if binary exists
  if (!fs.existsSync(binaryPath)) {
    console.log('⚠️  Binary not found. Compiling...');
    try {
      execSync('cargo build --release', { stdio: 'inherit' });
    } catch (error) {
      return {
        valid: false,
        error: 'Failed to compile binary',
        exitCode: 2
      };
    }
  }

  try {
    const output = execSync(
      `${binaryPath} --wallet ${walletType} --signature "${signature}" --address "${address}" --challenge "${challenge}"`,
      { encoding: 'utf8' }
    );

    return {
      valid: true,
      error: null,
      exitCode: 0,
      output: output
    };
  } catch (error) {
    return {
      valid: false,
      error: error.stderr?.toString() || error.message,
      exitCode: error.status || 1,
      output: error.stdout?.toString()
    };
  }
}

function main() {
  const args = process.argv.slice(2);

  if (args.length < 4 || args[0] === '--help' || args[0] === '-h') {
    console.log('🔐 XRPL Signature Verifier\n');
    console.log('Usage:');
    console.log('  node verify.js <wallet_type> <signature> <address> <challenge>\n');
    console.log('Examples:');
    console.log('  node verify.js xaman <hex_blob> <rAddress> <challenge>');
    console.log('  node verify.js web3auth <signature_hex> <rAddress> <challenge>\n');
    console.log('Supported wallets: xaman, web3auth');
    process.exit(0);
  }

  const [walletType, signature, address, challenge] = args;

  console.log('🔐 Verifying signature...\n');

  const result = verifySignature(walletType, signature, address, challenge);

  if (result.output) {
    console.log(result.output);
  }

  if (result.valid) {
    console.log('\n✅ VALID AUTHENTICATION');
    console.log('   User was successfully authenticated!');
    process.exit(0);
  } else if (result.exitCode === 1) {
    console.log('\n❌ AUTHENTICATION FAILED');
    console.log('   The signature is not valid.');
    process.exit(1);
  } else {
    console.log('\n⚠️  USAGE ERROR');
    if (result.error) {
      console.log(`   ${result.error}`);
    }
    process.exit(2);
  }
}

main();
