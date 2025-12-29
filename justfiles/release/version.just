# Show current version from Cargo.toml
# Usage: just version

version:
    @grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/'
