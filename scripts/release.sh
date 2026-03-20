#!/bin/bash
set -e

# Release helper script for FindeRS
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 2.1.1

if [ -z "$1" ]; then
    echo "❌ Error: Version number required"
    echo "Usage: ./scripts/release.sh <version>"
    echo "Example: ./scripts/release.sh 2.1.1"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

echo "🚀 Preparing release $VERSION"
echo ""

# Check if we're on main branch
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "⚠️  Warning: You're not on the main branch (currently on: $CURRENT_BRANCH)"
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Aborted."
        exit 1
    fi
fi

# Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ Error: You have uncommitted changes. Please commit or stash them first."
    git status --short
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "❌ Error: Tag $TAG already exists"
    exit 1
fi

echo "📝 Updating Cargo.toml version to $VERSION"
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm Cargo.toml.bak

echo "🔍 Verifying Cargo.toml compiles..."
cargo check --quiet

echo "✅ Running tests..."
cargo test --quiet

echo "📦 Building release binary..."
cargo build --release --quiet

CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2)
if [ "$CURRENT_VERSION" != "$VERSION" ]; then
    echo "❌ Error: Version update failed. Expected $VERSION, got $CURRENT_VERSION"
    exit 1
fi

echo ""
echo "✅ Pre-release checks passed!"
echo ""
echo "📋 Summary:"
echo "  Version: $VERSION"
echo "  Tag: $TAG"
echo "  Branch: $CURRENT_BRANCH"
echo ""
echo "Next steps:"
echo "  1. Update CHANGELOG.md with release notes for $VERSION"
echo "  2. Review changes: git diff"
echo "  3. Commit: git add Cargo.toml CHANGELOG.md && git commit -m 'Bump version to $VERSION'"
echo "  4. Tag: git tag $TAG"
echo "  5. Push: git push origin main && git push origin $TAG"
echo ""
echo "The release workflow will automatically:"
echo "  - Create a GitHub release with auto-generated notes"
echo "  - Publish to crates.io"
echo ""
read -p "Open CHANGELOG.md for editing? (Y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    ${EDITOR:-vim} CHANGELOG.md
fi
