# Bump patch version, publish to crates.io, and install locally
# Usage: just pub
# This will:
# 1. Commit any uncommitted changes
# 2. Bump the patch version
# 3. Create a git tag
# 4. Push to remote
# 5. Publish to crates.io
# 6. Install the new version locally

pub:
    #!/usr/bin/env bash
    set -euo pipefail
    
    # Check for ANY uncommitted changes
    echo "🔍 Checking git status..."
    if ! git diff --quiet --exit-code || ! git diff --cached --quiet --exit-code; then
        echo "⚠️  Found uncommitted changes. Committing all changes before release..."
        git add .
        git commit -m "chore: prepare for release"
        echo "✓ Committed all pending changes"
    else
        echo "✅ Git working directory is clean"
    fi
    
    # Get current version from Cargo.toml
    CURRENT_VERSION=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
    echo "Current version: $CURRENT_VERSION"
    
    # Parse version components
    MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
    MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
    PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)
    
    # Bump patch version
    NEW_PATCH=$((PATCH + 1))
    NEW_VERSION="${MAJOR}.${MINOR}.${NEW_PATCH}"
    echo "New version: $NEW_VERSION"
    
    # Update Cargo.toml (cross-platform sed)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
    else
        # Linux
        sed -i "s/^version = \"$CURRENT_VERSION\"/version = \"$NEW_VERSION\"/" Cargo.toml
    fi
    echo "✓ Updated Cargo.toml to version $NEW_VERSION"
    
    # Update Cargo.lock by running cargo check
    cargo check --quiet
    echo "✓ Updated Cargo.lock"
    
    # Git commit the version bump
    echo "📝 Committing version bump..."
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to $NEW_VERSION"
    echo "✓ Committed version bump"
    
    # Create git tag
    echo "🏷️  Creating git tag v$NEW_VERSION..."
    git tag "v$NEW_VERSION"
    echo "✓ Created tag v$NEW_VERSION"
    
    # Push commits and tags
    echo "🚀 Pushing to remote..."
    git push
    git push --tags
    echo "✓ Pushed commits and tags"
    
    # Publish to crates.io
    echo "📦 Publishing to crates.io..."
    cargo publish
    
    # Wait a bit for crates.io to process
    echo "⏳ Waiting 10 seconds for crates.io to process..."
    sleep 10
    
    # Install the new version
    echo "💿 Installing context-driven-development@$NEW_VERSION..."
    cargo install context-driven-development --version $NEW_VERSION --force
    
    echo "✅ Published and installed version $NEW_VERSION"
    echo "You can now run: cdd"
