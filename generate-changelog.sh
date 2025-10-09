#!/bin/bash
# Generate CHANGELOG.md using git-cliff

set -e

echo "🔨 Generating CHANGELOG.md..."

# Check if git-cliff is installed
if ! command -v git-cliff &> /dev/null; then
    echo "❌ git-cliff not found. Installing..."
    cargo install git-cliff
fi

# Generate changelog
git-cliff --output CHANGELOG.md

echo "✅ CHANGELOG.md generated successfully!"
echo ""
echo "📄 Preview:"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
head -n 30 CHANGELOG.md
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "💡 Tip: Review CHANGELOG.md and commit if looks good:"
echo "   git add CHANGELOG.md"
echo "   git commit -m 'chore: update CHANGELOG.md'"
