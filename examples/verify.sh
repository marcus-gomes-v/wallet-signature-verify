#!/bin/bash
# 🔐 Simple script to verify XRPL signatures

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo "🔐 XRPL Signature Verifier"
    echo ""
    echo "Usage:"
    echo "  $0 <wallet_type> <signature> <address> <challenge>"
    echo ""
    echo "Examples:"
    echo "  $0 xaman <hex_blob> <rAddress> <challenge>"
    echo "  $0 web3auth <signature_hex> <rAddress> <challenge>"
    echo ""
    echo "Supported wallets: xaman, web3auth"
    exit 0
}

# Check arguments
if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    show_help
fi

if [ $# -lt 4 ]; then
    echo -e "${RED}❌ Error: Insufficient arguments${NC}"
    echo ""
    show_help
fi

WALLET_TYPE=$1
SIGNATURE=$2
ADDRESS=$3
CHALLENGE=$4

# Detect script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BINARY="$PROJECT_DIR/target/release/wallet-signature-verify"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo -e "${YELLOW}⚠️  Binary not found. Compiling...${NC}"
    cd "$PROJECT_DIR" && cargo build --release

    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Compilation failed${NC}"
        exit 1
    fi
    cd - > /dev/null
fi

# Execute verification
echo "🔐 Verifying signature..."
echo ""

$BINARY \
    --wallet "$WALLET_TYPE" \
    --signature "$SIGNATURE" \
    --address "$ADDRESS" \
    --challenge "$CHALLENGE"

EXIT_CODE=$?

echo ""

# Interpret result
if [ $EXIT_CODE -eq 0 ]; then
    echo -e "${GREEN}✅ VALID AUTHENTICATION${NC}"
    echo "   User was successfully authenticated!"
    exit 0
elif [ $EXIT_CODE -eq 1 ]; then
    echo -e "${RED}❌ AUTHENTICATION FAILED${NC}"
    echo "   The signature is not valid."
    exit 1
else
    echo -e "${YELLOW}⚠️  USAGE ERROR${NC}"
    echo "   Check the arguments."
    exit 2
fi
