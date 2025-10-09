#!/usr/bin/env python3
"""
🔐 XRPL Signature Verifier - Python Example

Usage:
    python3 verify.py xaman <signature> <address> <challenge>
    python3 verify.py web3auth <signature> <address> <challenge>
"""

import subprocess
import sys
from pathlib import Path

def verify_signature(wallet_type: str, signature: str, address: str, challenge: str) -> dict:
    """
    Verify XRPL signature using the Rust binary.

    Args:
        wallet_type: Wallet type ("xaman" or "web3auth")
        signature: Signature hex
        address: XRP address (rAddress format)
        challenge: Challenge string

    Returns:
        dict with keys:
            - valid (bool): If the signature is valid
            - error (str|None): Error message if any
            - exit_code (int): Process exit code
    """
    binary_path = Path("./target/release/wallet-signature-verify")

    # Check if binary exists
    if not binary_path.exists():
        print("⚠️  Binary not found. Compiling...")
        compile_result = subprocess.run(["cargo", "build", "--release"], capture_output=True)
        if compile_result.returncode != 0:
            return {
                "valid": False,
                "error": "Failed to compile binary",
                "exit_code": 2
            }

    try:
        result = subprocess.run(
            [
                str(binary_path),
                "--wallet", wallet_type,
                "--signature", signature,
                "--address", address,
                "--challenge", challenge
            ],
            capture_output=True,
            text=True,
            check=False  # Don't throw exception on exit code != 0
        )

        return {
            "valid": result.returncode == 0,
            "error": result.stderr if result.returncode != 0 else None,
            "exit_code": result.returncode,
            "output": result.stdout
        }
    except Exception as e:
        return {
            "valid": False,
            "error": str(e),
            "exit_code": 2
        }

def main():
    if len(sys.argv) < 5 or sys.argv[1] in ["--help", "-h"]:
        print("🔐 XRPL Signature Verifier\n")
        print("Usage:")
        print(f"  {sys.argv[0]} <wallet_type> <signature> <address> <challenge>\n")
        print("Examples:")
        print(f"  {sys.argv[0]} xaman <hex_blob> <rAddress> <challenge>")
        print(f"  {sys.argv[0]} web3auth <signature_hex> <rAddress> <challenge>\n")
        print("Supported wallets: xaman, web3auth")
        sys.exit(0)

    wallet_type = sys.argv[1]
    signature = sys.argv[2]
    address = sys.argv[3]
    challenge = sys.argv[4]

    print("🔐 Verifying signature...\n")

    result = verify_signature(wallet_type, signature, address, challenge)

    print(result["output"])

    if result["valid"]:
        print("\n✅ VALID AUTHENTICATION")
        print("   User was successfully authenticated!")
        sys.exit(0)
    elif result["exit_code"] == 1:
        print("\n❌ AUTHENTICATION FAILED")
        print("   The signature is not valid.")
        sys.exit(1)
    else:
        print("\n⚠️  USAGE ERROR")
        if result["error"]:
            print(f"   {result['error']}")
        sys.exit(2)

if __name__ == "__main__":
    main()
